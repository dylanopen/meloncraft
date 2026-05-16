use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndStoneBrickWall {
    pub waterlogged: bool,
    pub r#west: West,
    pub up: bool,
    pub r#north: North,
    pub r#south: South,
    pub r#east: East,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum East {
    None,
    Low,
    Tall,
}

impl BlockState for EndStoneBrickWall {
    fn to_id(&self) -> i32 {
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#up == true && self.r#south == South::Tall { return 20137; }
        if self.r#up == true && self.r#east == East::Low && self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low { return 20053; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::None && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == true { return 20115; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall && self.r#south == South::Low { return 20014; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::None { return 19892; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#up == false && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::None { return 19876; }
        if self.r#north == North::Tall && self.r#up == true && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::None { return 19933; }
        if self.r#east == East::None && self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::Low { return 19918; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall { return 19943; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Tall && self.r#up == false && self.r#south == South::Low { return 20054; }
        if self.r#south == South::Tall && self.r#up == false && self.r#north == North::Low && self.r#west == West::Tall && self.r#east == East::None && self.r#waterlogged == false { return 19927; }
        if self.r#up == true && self.r#north == North::Low && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Low { return 20003; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::None && self.r#west == West::None && self.r#up == false && self.r#east == East::None { return 19865; }
        if self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::Tall { return 19890; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == true { return 20127; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == false { return 20047; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::None { return 19870; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 20038; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#up == true { return 19976; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Low && self.r#up == false && self.r#south == South::Low && self.r#west == West::Tall { return 20020; }
        if self.r#waterlogged == true && self.r#up == false && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Low && self.r#west == West::None { return 20018; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low && self.r#up == false && self.r#east == East::Tall && self.r#north == North::Tall { return 20165; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::None && self.r#up == true && self.r#west == West::Low { return 19968; }
        if self.r#north == North::None && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Low { return 19974; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false && self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::Tall { return 20103; }
        if self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::Low { return 19915; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Low { return 20157; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false && self.r#south == South::Tall { return 20102; }
        if self.r#west == West::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::None && self.r#up == true { return 20005; }
        if self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true { return 20098; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Tall && self.r#west == West::Tall && self.r#up == false && self.r#north == North::None { return 19888; }
        if self.r#up == false && self.r#north == North::Tall && self.r#east == East::None && self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::None { return 19935; }
        if self.r#up == true && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::None { return 19991; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::None { return 19928; }
        if self.r#east == East::Low && self.r#up == false && self.r#north == North::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Low { return 20057; }
        if self.r#up == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Low { return 20109; }
        if self.r#up == true && self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::Low { return 20110; }
        if self.r#west == West::None && self.r#north == North::None && self.r#south == South::None && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true { return 19856; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#north == North::None && self.r#up == true && self.r#south == South::None && self.r#waterlogged == false { return 19967; }
        if self.r#east == East::None && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == false { return 19891; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Low { return 19966; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#up == false && self.r#north == North::Tall { return 20176; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::None && self.r#up == false { return 20118; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low { return 20040; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::None { return 20009; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#up == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == true { return 20128; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None && self.r#up == false && self.r#east == East::None && self.r#north == North::Low { return 19901; }
        if self.r#up == false && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::Tall { return 19963; }
        if self.r#west == West::None && self.r#north == North::None && self.r#east == East::Low && self.r#up == true && self.r#south == South::None && self.r#waterlogged == true { return 19964; }
        if self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::None && self.r#east == East::None { return 19900; }
        if self.r#south == South::Tall && self.r#up == true && self.r#north == North::Low && self.r#east == East::None && self.r#west == West::Low && self.r#waterlogged == true { return 19917; }
        if self.r#west == West::Tall && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Tall { return 20116; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Tall { return 20172; }
        if self.r#south == South::Low && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true { return 19940; }
        if self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::None && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == false { return 19957; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::None { return 19907; }
        if self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Low { return 19904; }
        if self.r#up == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == false { return 19981; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Low { return 19984; }
        if self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::None && self.r#up == false { return 20081; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::None && self.r#up == false { return 19877; }
        if self.r#west == West::None && self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true { return 19934; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#up == false && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == false { return 20105; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#south == South::None { return 20080; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#west == West::None && self.r#waterlogged == true && self.r#up == false && self.r#south == South::Low { return 19910; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#up == false && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == false { return 19879; }
        if self.r#up == true && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Tall { return 20089; }
        if self.r#up == false && self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Tall { return 20179; }
        if self.r#north == North::None && self.r#east == East::None && self.r#up == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == false { return 19885; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::None { return 19887; }
        if self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::None && self.r#up == true && self.r#west == West::Tall { return 19942; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::None { return 20087; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true { return 20050; }
        if self.r#waterlogged == false && self.r#up == true && self.r#north == North::Low && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::None { return 20111; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Low { return 19905; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#east == East::None && self.r#west == West::Tall && self.r#waterlogged == false { return 19903; }
        if self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::Tall { return 20069; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::Low && self.r#west == West::Low { return 19908; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::Tall { return 20063; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::None { return 19997; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == true && self.r#east == East::Low && self.r#west == West::None { return 20048; }
        if self.r#north == North::None && self.r#west == West::Low && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == true { return 20091; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == true && self.r#east == East::Low && self.r#west == West::Low { return 20049; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::Low && self.r#north == North::Low && self.r#up == false && self.r#west == West::None { return 20021; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#west == West::None && self.r#waterlogged == false && self.r#up == false && self.r#north == North::Low { return 20117; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Low && self.r#east == East::None && self.r#waterlogged == false { return 19926; }
        if self.r#up == true && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Tall { return 20122; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Low && self.r#up == false && self.r#north == North::None && self.r#west == West::Tall { return 20095; }
        if self.r#up == false && self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Tall { return 20107; }
        if self.r#south == South::Low && self.r#up == true && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == true { return 20084; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Tall { return 20145; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::None { return 20149; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false && self.r#west == West::Low { return 20163; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#west == West::Tall && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true { return 20008; }
        if self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true { return 20164; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == true && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::Tall { return 20168; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#east == East::Tall && self.r#west == West::None && self.r#up == true && self.r#waterlogged == false { return 20171; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#up == true && self.r#west == West::Tall && self.r#south == South::None && self.r#waterlogged == false { return 20077; }
        if self.r#up == false && self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::None { return 20114; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Tall && self.r#north == North::Low { return 20023; }
        if self.r#up == true && self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::Low { return 20001; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Tall { return 20056; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#up == true && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::None { return 19871; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low { return 20112; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::Tall && self.r#west == West::Tall && self.r#north == North::Tall && self.r#up == false { return 20167; }
        if self.r#north == North::Tall && self.r#up == true && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall { return 20173; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::None && self.r#up == false { return 19971; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::Low { return 20028; }
        if self.r#up == true && self.r#west == West::None && self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false { return 19859; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Low && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == false { return 20027; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == false && self.r#south == South::Tall { return 20071; }
        if self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low { return 20094; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true && self.r#north == North::Low { return 19906; }
        if self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Tall && self.r#up == false { return 20141; }
        if self.r#south == South::Tall && self.r#up == true && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == false { return 20064; }
        if self.r#up == false && self.r#south == South::Low && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == true { return 20162; }
        if self.r#east == East::Tall && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == true { return 20170; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::None && self.r#up == true { return 19919; }
        if self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Tall && self.r#west == West::Low { return 19932; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#up == false && self.r#south == South::Tall && self.r#west == West::Low { return 19995; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true && self.r#west == West::None && self.r#east == East::Tall { return 20159; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall { return 20166; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::None { return 20174; }
        if self.r#south == South::Low && self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::None && self.r#up == false { return 19985; }
        if self.r#north == North::None && self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == true { return 19884; }
        if self.r#west == West::Tall && self.r#up == true && self.r#south == South::Low && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::None { return 19909; }
        if self.r#north == North::Low && self.r#up == false && self.r#south == South::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Low { return 19911; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Tall { return 19921; }
        if self.r#north == North::Tall && self.r#up == true && self.r#west == West::None && self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == false { return 19955; }
        if self.r#up == false && self.r#west == West::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::Low { return 19923; }
        if self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false && self.r#north == North::Tall && self.r#east == East::None { return 19947; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall { return 20070; }
        if self.r#up == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::None { return 20073; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::None && self.r#up == true && self.r#south == South::None && self.r#east == East::None { return 19895; }
        if self.r#north == North::None && self.r#south == South::None && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None { return 20075; }
        if self.r#south == South::None && self.r#north == North::None && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Tall { return 20082; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall { return 20092; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#west == West::Tall && self.r#up == false && self.r#south == South::None && self.r#waterlogged == false { return 20119; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Tall { return 20139; }
        if self.r#west == West::None && self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false { return 19937; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Low { return 20013; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == true && self.r#east == East::Tall { return 20169; }
        if self.r#up == false && self.r#north == North::Tall && self.r#south == South::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Low { return 20043; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Low && self.r#up == true && self.r#west == West::Low { return 20121; }
        if self.r#west == West::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::None { return 20076; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::None && self.r#up == true { return 20099; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Low && self.r#east == East::None && self.r#up == false { return 19902; }
        if self.r#waterlogged == true && self.r#up == false && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::None && self.r#west == West::Tall { return 20152; }
        if self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::None { return 20036; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall { return 20062; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Tall { return 20030; }
        if self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::Tall { return 19989; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Low && self.r#up == true { return 20024; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Tall { return 20177; }
        if self.r#up == false && self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::None { return 19936; }
        if self.r#east == East::Low && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false { return 19969; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low && self.r#up == true { return 19977; }
        if self.r#south == South::Tall && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Low { return 20025; }
        if self.r#up == false && self.r#west == West::Low && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Tall { return 19959; }
        if self.r#north == North::Low && self.r#west == West::None && self.r#east == East::None && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true { return 19922; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::None { return 20088; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Low && self.r#up == false { return 19987; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::Low && self.r#west == West::Tall && self.r#up == true { return 20125; }
        if self.r#south == South::Low && self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == false { return 20131; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Low && self.r#up == true && self.r#waterlogged == false { return 20160; }
        if self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::Tall && self.r#west == West::Tall { return 19930; }
        if self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Low { return 20061; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::Low { return 20022; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Tall && self.r#west == West::None && self.r#waterlogged == true && self.r#up == true { return 20132; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::None && self.r#up == true { return 20074; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false && self.r#west == West::None && self.r#east == East::None && self.r#waterlogged == false { return 19949; }
        if self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false { return 20017; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::None && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == true { return 20060; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Low { return 20130; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::None && self.r#up == false && self.r#east == East::Low { return 20044; }
        if self.r#south == South::None && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Tall { return 19929; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#up == false && self.r#south == South::None && self.r#north == North::None && self.r#west == West::None { return 19862; }
        if self.r#up == false && self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::None && self.r#east == East::None { return 19946; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None { return 19982; }
        if self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == true { return 19863; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#up == false && self.r#east == East::None { return 19898; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall { return 20134; }
        if self.r#east == East::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Low { return 19912; }
        if self.r#west == West::None && self.r#south == South::None && self.r#east == East::Low && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == false { return 20039; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#up == false && self.r#south == South::None { return 20154; }
        if self.r#up == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::Tall && self.r#east == East::None && self.r#waterlogged == false { return 19962; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Tall { return 19945; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#up == false && self.r#west == West::None && self.r#south == South::Tall { return 19961; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#west == West::None && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low { return 20000; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None && self.r#east == East::None && self.r#south == South::Low && self.r#up == false { return 19878; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Tall { return 19953; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#up == false && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Low { return 20019; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false && self.r#south == South::None { return 20151; }
        if self.r#east == East::None && self.r#south == South::None && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall { return 19939; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == true { return 20133; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::None && self.r#up == true { return 20004; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#up == true && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Tall { return 19978; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Low { return 20012; }
        if self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::Tall { return 20097; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::None && self.r#up == true { return 19965; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall { return 20029; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#up == false && self.r#south == South::None && self.r#north == North::None && self.r#waterlogged == false { return 19973; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Low && self.r#up == false { return 19999; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Low && self.r#up == false && self.r#west == West::Low && self.r#south == South::Tall { return 20142; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::Low { return 20086; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall { return 20150; }
        if self.r#west == West::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#up == false && self.r#east == East::None && self.r#south == South::Low { return 19875; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::Low && self.r#east == East::Low && self.r#up == false && self.r#west == West::None { return 20006; }
        if self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Tall && self.r#west == West::Low { return 19956; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::None && self.r#up == false { return 20007; }
        if self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low { return 20037; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false { return 20041; }
        if self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Tall && self.r#west == West::None { return 20066; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None && self.r#up == false && self.r#east == East::Low { return 20045; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::None && self.r#up == true { return 20072; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::None && self.r#east == East::Tall && self.r#up == true { return 20156; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::None && self.r#up == false { return 19874; }
        if self.r#east == East::Tall && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::Low { return 20140; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#up == true && self.r#west == West::Low { return 19920; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == false { return 20136; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::None { return 19938; }
        if self.r#east == East::None && self.r#up == false && self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::Tall { return 19886; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#north == North::None && self.r#up == true && self.r#east == East::None && self.r#south == South::Low { return 19869; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::None && self.r#up == true && self.r#west == West::None { return 19916; }
        if self.r#up == false && self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::None { return 19994; }
        if self.r#south == South::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Low && self.r#up == false { return 19899; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false && self.r#south == South::None && self.r#west == West::None && self.r#east == East::Tall { return 20153; }
        if self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false { return 20113; }
        if self.r#west == West::Low && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Low { return 20031; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Low { return 19972; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true { return 19881; }
        if self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::None { return 19992; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false { return 20079; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#west == West::None && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false { return 19913; }
        if self.r#north == North::Low && self.r#up == true && self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Tall { return 20108; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == true && self.r#north == North::Tall && self.r#east == East::None { return 19952; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#east == East::None && self.r#up == true && self.r#south == South::None && self.r#waterlogged == false { return 19861; }
        if self.r#up == false && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Low { return 20178; }
        if self.r#east == East::Tall && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::None { return 20083; }
        if self.r#west == West::Low && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::None { return 19983; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#north == North::None && self.r#up == true { return 19883; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::None && self.r#west == West::Low && self.r#up == true && self.r#north == North::Low { return 19896; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Low { return 20058; }
        if self.r#north == North::Tall && self.r#up == false && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == true { return 20042; }
        if self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false { return 20059; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::Low && self.r#up == true && self.r#north == North::None && self.r#south == South::Low { return 20085; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall { return 20096; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Tall { return 19941; }
        if self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::Tall { return 20032; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::None && self.r#west == West::Tall { return 20002; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None { return 19931; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 20138; }
        if self.r#south == South::None && self.r#west == West::Tall && self.r#waterlogged == false && self.r#up == false && self.r#east == East::Low && self.r#north == North::Low { return 20011; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#west == West::Tall && self.r#up == true && self.r#east == East::None && self.r#waterlogged == true { return 19882; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::None && self.r#up == false && self.r#west == West::Low && self.r#north == North::None { return 19866; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::None && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false { return 19950; }
        if self.r#north == North::Low && self.r#up == false && self.r#south == South::Low && self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Tall { return 20126; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#north == North::None { return 19970; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#up == false && self.r#south == South::Tall { return 19925; }
        if self.r#south == South::None && self.r#up == true && self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None { return 20147; }
        if self.r#up == false && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Low { return 19998; }
        if self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Low && self.r#north == North::Tall && self.r#west == West::Tall { return 20161; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#up == true && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Low { return 20016; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low && self.r#east == East::Low { return 20052; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#up == false { return 19889; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#up == false && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == false { return 19975; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#up == false && self.r#west == West::Low && self.r#south == South::Low { return 19914; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#up == true && self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::None { return 19857; }
        if self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::None && self.r#west == West::None { return 19868; }
        if self.r#north == North::None && self.r#up == true && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Tall && self.r#east == East::None { return 19873; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#up == true { return 19872; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Low && self.r#up == false { return 20067; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#waterlogged == true && self.r#up == true && self.r#east == East::None && self.r#south == South::None { return 19858; }
        if self.r#up == true && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::None { return 19980; }
        if self.r#south == South::None && self.r#north == North::Tall && self.r#up == false && self.r#west == West::Tall && self.r#east == East::Tall && self.r#waterlogged == false { return 20155; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == false && self.r#east == East::Low { return 20068; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Low { return 19944; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 20101; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#up == false && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == false { return 20035; }
        if self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Low { return 20055; }
        if self.r#south == South::Low && self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true { return 19948; }
        if self.r#waterlogged == false && self.r#up == false && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#west == West::Tall { return 19951; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#up == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == true { return 19960; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low && self.r#up == false { return 20093; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == false && self.r#up == false && self.r#west == West::Low { return 20010; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None && self.r#east == East::Low { return 20033; }
        if self.r#west == West::None && self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == false && self.r#north == North::Tall { return 19958; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall { return 19993; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == false && self.r#up == false && self.r#west == West::Low { return 20046; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Low && self.r#up == true { return 20051; }
        if self.r#south == South::None && self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 19864; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low { return 20065; }
        if self.r#up == true && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Tall { return 20120; }
        if self.r#east == East::Low && self.r#up == false && self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Tall { return 19996; }
        if self.r#up == true && self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Tall && self.r#north == North::None { return 19880; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == true && self.r#west == West::None { return 20123; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall { return 20104; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low { return 20175; }
        if self.r#up == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::Tall { return 20148; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false { return 20034; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None { return 20144; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::None && self.r#up == false && self.r#west == West::Tall { return 19867; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall { return 19954; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true { return 20135; }
        if self.r#north == North::Low && self.r#up == true && self.r#west == West::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Tall { return 20124; }
        if self.r#waterlogged == false && self.r#up == true && self.r#north == North::None && self.r#east == East::None && self.r#west == West::Low && self.r#south == South::None { return 19860; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#up == true && self.r#south == South::None && self.r#west == West::Tall { return 19894; }
        if self.r#north == North::None && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall { return 20090; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::Low && self.r#up == false && self.r#west == West::Tall && self.r#east == East::None { return 19924; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#north == North::Low && self.r#east == East::None && self.r#up == true { return 19893; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::Low && self.r#up == true && self.r#west == West::None && self.r#east == East::Low { return 20015; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Tall && self.r#south == South::Tall && self.r#up == true { return 20026; }
        if self.r#up == true && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Tall { return 19988; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false { return 19986; }
        if self.r#up == true && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall { return 20100; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false { return 20106; }
        if self.r#up == true && self.r#east == East::Tall && self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::None { return 20146; }
        if self.r#north == North::Tall && self.r#up == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Tall && self.r#waterlogged == true { return 20158; }
        if self.r#west == West::None && self.r#up == false && self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::None && self.r#waterlogged == true { return 20078; }
        if self.r#up == true && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == false { return 19897; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low { return 19979; }
        if self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == false { return 20143; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low { return 20129; }
        if self.r#north == North::None && self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Low { return 19990; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20137 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 20053 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 20115 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20014 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 19892 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 19876 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 19933 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19918 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19943 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 20054 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 19927 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20003 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 19865 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 19890 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 20127 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20047 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19870 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 20038 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19976 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 20020 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 20018 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 20165 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19968 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 19974 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 20103 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 19915 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 20157 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 20102 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 20005 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 20098 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19888 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 19935 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 19991 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 19928 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 20057 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 20109 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 20110 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 19856 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19967 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19891 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19966 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 20176 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 20118 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 20040 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 20009 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 20128 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19901 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 19963 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19964 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19900 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 19917 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20116 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 20172 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19940 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19957 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19907 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 19904 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 19981 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19984 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 20081 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 19877 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 19934 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 20105 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20080 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 19910 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 19879 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20089 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 20179 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19885 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19887 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 19942 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20087 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 20050 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 20111 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19905 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 19903 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20069 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 19908 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 20063 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 19997 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 20048 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 20091 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20049 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 20021 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 20117 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 19926 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20122 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 20095 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 20107 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 20084 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20145 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 20149 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 20163 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 20008 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20164 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20168 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 20171 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20077 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20114 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 20023 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 20001 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 20056 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19871 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 20112 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 20167 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 20173 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19971 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 20028 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 19859 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20027 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 20071 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 20094 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 19906 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 20141 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 20064 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20162 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20170 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19919 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 19932 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 19995 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 20159 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 20166 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 20174 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19985 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 19884 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 19909 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 19911 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19921 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19955 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19923 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19947 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 20070 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 20073 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 19895 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 20075 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 20082 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 20092 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 20119 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20139 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19937 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 20013 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 20169 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 20043 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 20121 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 20076 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 20099 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19902 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 20152 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 20036 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 20062 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 20030 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19989 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 20024 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 20177 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 19936 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 19969 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19977 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 20025 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 19959 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19922 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20088 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19987 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 20125 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 20131 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 20160 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19930 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 20061 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 20022 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 20132 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 20074 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 19949 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20017 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20060 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 20130 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 20044 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 19929 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19862 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 19946 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 19982 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19863 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19898 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 20134 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 19912 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 20039 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20154 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 19962 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19945 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19961 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 20000 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 19878 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 19953 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 20019 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 20151 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 19939 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 20133 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 20004 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 19978 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20012 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 20097 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19965 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 20029 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19973 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19999 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 20142 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 20086 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 20150 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 19875 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 20006 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 19956 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 20007 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 20037 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 20041 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20066 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 20045 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 20072 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 20156 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 19874 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 20140 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19920 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 20136 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19938 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 19886 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19869 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 19916 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 19994 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19899 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 20153 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 20113 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 20031 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19972 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 19881 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19992 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 20079 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 19913 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20108 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 19952 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 19861 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 20178 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 20083 {
            return Some(EndStoneBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 19983 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 19883 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19896 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 20058 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 20042 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 20059 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20085 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 20096 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19941 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 20032 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 20002 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Tall,
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
        if state_id == 20138 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 20011 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 19882 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19866 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 19950 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20126 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 19970 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 19925 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 20147 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19998 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 20161 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 20016 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 20052 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19889 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 19975 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19914 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19857 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 19868 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 19873 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 19872 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 20067 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 19858 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 19980 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 20155 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20068 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 19944 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 20101 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 20035 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 20055 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 19948 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19951 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 19960 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20093 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 20010 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 20033 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 19958 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 19993 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 20046 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 20051 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 19864 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 20065 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 20120 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 19996 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19880 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 20123 {
            return Some(EndStoneBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 20104 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 20175 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 20148 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 20034 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 20144 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 19867 {
            return Some(EndStoneBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19954 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 20135 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 20124 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 19860 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 19894 {
            return Some(EndStoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 20090 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19924 {
            return Some(EndStoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 19893 {
            return Some(EndStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 20015 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 20026 {
            return Some(EndStoneBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 19988 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19986 {
            return Some(EndStoneBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 20100 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 20106 {
            return Some(EndStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 20146 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 20158 {
            return Some(EndStoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 20078 {
            return Some(EndStoneBrickWall {
                r#west: West::None,
                r#up: false,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19897 {
            return Some(EndStoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19979 {
            return Some(EndStoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 20143 {
            return Some(EndStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 20129 {
            return Some(EndStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19990 {
            return Some(EndStoneBrickWall {
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        return None;
    }
}

