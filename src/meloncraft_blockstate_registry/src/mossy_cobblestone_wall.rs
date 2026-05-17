use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossyCobblestoneWall {
    pub r#west: West,
    pub waterlogged: bool,
    pub r#east: East,
    pub r#north: North,
    pub up: bool,
    pub r#south: South,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum South {
    None,
    Low,
    Tall,
}

impl BlockState for MossyCobblestoneWall {
    fn to_id(&self) -> i32 {
        if self.r#west == West::Low && self.r#up == true && self.r#east == East::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Low { return 10153; }
        if self.r#south == South::Tall && self.r#up == false && self.r#west == West::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Tall { return 10314; }
        if self.r#west == West::Tall && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::Low { return 10358; }
        if self.r#east == East::Low && self.r#west == West::Tall && self.r#waterlogged == false && self.r#up == false && self.r#south == South::None && self.r#north == North::Low { return 10259; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::None && self.r#west == West::Tall && self.r#waterlogged == false && self.r#up == true { return 10325; }
        if self.r#up == true && self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low { return 10345; }
        if self.r#east == East::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#up == false && self.r#south == South::None { return 10365; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::Tall && self.r#up == false && self.r#east == East::Tall { return 10399; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true && self.r#south == South::Tall { return 10421; }
        if self.r#south == South::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::None { return 10125; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::None && self.r#north == North::Tall && self.r#up == true { return 10287; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::Tall && self.r#up == false && self.r#north == North::None { return 10331; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Tall { return 10343; }
        if self.r#south == South::None && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::None { return 10401; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Tall { return 10245; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::Low && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true { return 10242; }
        if self.r#north == North::Low && self.r#west == West::None && self.r#south == South::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == false { return 10266; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == false && self.r#west == West::None && self.r#south == South::Low { return 10410; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::None && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true { return 10147; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None { return 10326; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Low && self.r#up == false { return 10271; }
        if self.r#waterlogged == true && self.r#up == false && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::Tall && self.r#west == West::Low { return 10351; }
        if self.r#waterlogged == false && self.r#up == true && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::None && self.r#west == West::Low { return 10156; }
        if self.r#west == West::None && self.r#up == false && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == false { return 10305; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#up == true && self.r#north == North::Tall && self.r#south == South::None && self.r#east == East::None { return 10176; }
        if self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::None { return 10183; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#up == true && self.r#west == West::None && self.r#north == North::None && self.r#east == East::None { return 10104; }
        if self.r#up == false && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::None && self.r#west == West::Tall && self.r#east == East::None { return 10148; }
        if self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::None { return 10177; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::None && self.r#up == true && self.r#west == West::Low && self.r#north == North::Low { return 10144; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true { return 10159; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::None && self.r#up == true && self.r#west == West::None { return 10179; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == false && self.r#west == West::Low && self.r#east == East::None { return 10198; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#west == West::None && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false { return 10251; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Tall { return 10286; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::Tall { return 10304; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true { return 10420; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#up == false && self.r#east == East::Low && self.r#south == South::None && self.r#west == West::Tall { return 10223; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::None && self.r#up == true { return 10299; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#east == East::None && self.r#up == true && self.r#south == South::Tall && self.r#north == North::Low { return 10165; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Low { return 10312; }
        if self.r#up == false && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Tall && self.r#north == North::Tall { return 10415; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Low && self.r#up == true && self.r#north == North::None { return 10240; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == true && self.r#north == North::None { return 10340; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::None && self.r#east == East::None && self.r#up == true && self.r#south == South::Tall { return 10200; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == false && self.r#east == East::None && self.r#south == South::Low && self.r#west == West::None { return 10194; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::None && self.r#west == West::Low && self.r#east == East::None && self.r#up == true { return 10108; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::None && self.r#south == South::Tall { return 10275; }
        if self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None { return 10294; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Low && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true { return 10302; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::None { return 10115; }
        if self.r#east == East::None && self.r#south == South::None && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 10184; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#east == East::None { return 10204; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall { return 10295; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::None && self.r#south == South::Tall && self.r#up == false { return 10425; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false { return 10354; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == false && self.r#south == South::Low { return 10376; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#up == true { return 10289; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall { return 10414; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::Tall { return 10211; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#south == South::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false { return 10180; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true { return 10303; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#west == West::Tall && self.r#south == South::None { return 10292; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Low && self.r#up == true { return 10369; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true { return 10328; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#up == false && self.r#south == South::Tall { return 10386; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 10226; }
        if self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Tall { return 10209; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Low && self.r#up == true && self.r#south == South::Tall && self.r#west == West::Tall { return 10385; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Tall { return 10283; }
        if self.r#up == false && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::None { return 10137; }
        if self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::Tall && self.r#west == West::Tall && self.r#south == South::Tall { return 10418; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#up == false && self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == false { return 10187; }
        if self.r#waterlogged == true && self.r#up == false && self.r#north == North::Tall && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Low { return 10316; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Tall { return 10342; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall { return 10356; }
        if self.r#west == West::Tall && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Low { return 10370; }
        if self.r#west == West::Low && self.r#up == true && self.r#north == North::Tall && self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::Low { return 10297; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#up == true && self.r#west == West::Tall { return 10106; }
        if self.r#south == South::Tall && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::Low { return 10387; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#up == true && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::None { return 10296; }
        if self.r#up == false && self.r#south == South::Tall && self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Low { return 10426; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#up == true && self.r#west == West::None && self.r#south == South::Low && self.r#east == East::Low { return 10227; }
        if self.r#north == North::None && self.r#up == false && self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Low { return 10111; }
        if self.r#north == North::Low && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::Low && self.r#up == true { return 10260; }
        if self.r#north == North::None && self.r#up == true && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None { return 10119; }
        if self.r#up == true && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Low { return 10189; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#up == false && self.r#west == West::None { return 10422; }
        if self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Low && self.r#east == East::None { return 10149; }
        if self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::Tall { return 10315; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == false && self.r#east == East::None && self.r#north == North::None { return 10136; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == false && self.r#east == East::Low && self.r#north == North::Low { return 10282; }
        if self.r#west == West::Tall && self.r#up == false && self.r#south == South::None && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == true { return 10112; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#east == East::Low { return 10253; }
        if self.r#south == South::None && self.r#west == West::None && self.r#north == North::None && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false { return 10323; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Low { return 10255; }
        if self.r#west == West::None && self.r#up == true && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::Low { return 10332; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#west == West::None && self.r#up == true && self.r#waterlogged == true && self.r#north == North::None { return 10128; }
        if self.r#west == West::None && self.r#north == North::None && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::None { return 10221; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#east == East::None && self.r#west == West::None && self.r#waterlogged == true { return 10134; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Low && self.r#south == South::Tall { return 10273; }
        if self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#south == South::None { return 10288; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 10268; }
        if self.r#up == true && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::Low { return 10193; }
        if self.r#north == North::None && self.r#west == West::None && self.r#up == false && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == true { return 10218; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::None && self.r#up == true && self.r#west == West::Low && self.r#south == South::Low { return 10336; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == true { return 10406; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#up == true && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::Low { return 10117; }
        if self.r#south == South::Tall && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::None { return 10352; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false { return 10317; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false { return 10307; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::Low { return 10377; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None { return 10344; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#up == false { return 10353; }
        if self.r#west == West::Low && self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == false && self.r#up == false && self.r#north == North::Low { return 10150; }
        if self.r#up == false && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#west == West::Tall { return 10280; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#east == East::None && self.r#up == true && self.r#west == West::None && self.r#waterlogged == false { return 10203; }
        if self.r#west == West::None && self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::None { return 10122; }
        if self.r#up == true && self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Low { return 10141; }
        if self.r#north == North::None && self.r#up == false && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::None { return 10230; }
        if self.r#south == South::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#up == true && self.r#west == West::Tall { return 10394; }
        if self.r#north == North::None && self.r#up == true && self.r#south == South::Tall && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Tall { return 10241; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 10116; }
        if self.r#up == true && self.r#north == North::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#west == West::None && self.r#waterlogged == true { return 10380; }
        if self.r#up == false && self.r#south == South::None && self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Tall { return 10403; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == true { return 10407; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Low && self.r#up == false && self.r#south == South::Low && self.r#east == East::None { return 10161; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::None { return 10244; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::Tall && self.r#up == false && self.r#west == West::None { return 10338; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == true { return 10408; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == true && self.r#east == East::None && self.r#west == West::Low && self.r#north == North::None { return 10132; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#up == false && self.r#south == South::Low && self.r#west == West::None && self.r#east == East::Low { return 10233; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#up == false { return 10281; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#west == West::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true { return 10206; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == true && self.r#west == West::Tall && self.r#north == North::None && self.r#waterlogged == true { return 10322; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == true && self.r#south == South::Low && self.r#north == North::Low { return 10261; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#up == false && self.r#south == South::Low { return 10235; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == true { return 10192; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Low && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == false { return 10300; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#east == East::None && self.r#waterlogged == false && self.r#up == true && self.r#north == North::None { return 10131; }
        if self.r#up == true && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None { return 10239; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == true && self.r#south == South::Tall { return 10308; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::None && self.r#west == West::Low && self.r#up == true { return 10129; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::Low && self.r#up == true && self.r#east == East::Tall && self.r#west == West::None { return 10359; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#west == West::Low && self.r#south == South::Tall { return 10135; }
        if self.r#east == East::None && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::None { return 10167; }
        if self.r#waterlogged == true && self.r#up == false && self.r#north == North::Low && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Low { return 10267; }
        if self.r#east == East::None && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false && self.r#south == South::Low && self.r#north == North::None { return 10126; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::None && self.r#south == South::None { return 10109; }
        if self.r#waterlogged == false && self.r#up == true && self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Low && self.r#west == West::Tall { return 10229; }
        if self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Tall { return 10348; }
        if self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false { return 10372; }
        if self.r#south == South::None && self.r#west == West::None && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::None { return 10320; }
        if self.r#up == true && self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::Tall && self.r#west == West::Low && self.r#waterlogged == true { return 10381; }
        if self.r#south == South::Tall && self.r#up == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#west == West::Low && self.r#waterlogged == false { return 10318; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false { return 10252; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#up == true && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == false { return 10311; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == true && self.r#south == South::None && self.r#north == North::Low { return 10360; }
        if self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#north == North::Tall { return 10398; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::None && self.r#west == West::Tall && self.r#waterlogged == false && self.r#up == true { return 10337; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#west == West::None && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Low { return 10404; }
        if self.r#up == false && self.r#east == East::None && self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == true { return 10207; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None { return 10133; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#waterlogged == true { return 10232; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#up == true && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::None { return 10284; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#east == East::Tall { return 10361; }
        if self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false { return 10168; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::None && self.r#west == West::None && self.r#north == North::None && self.r#up == true { return 10212; }
        if self.r#west == West::None && self.r#south == South::None && self.r#north == North::Low && self.r#up == true && self.r#east == East::None && self.r#waterlogged == false { return 10143; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Low { return 10375; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::None { return 10400; }
        if self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Tall { return 10174; }
        if self.r#up == false && self.r#north == North::Tall && self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Low { return 10196; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::Low && self.r#up == true && self.r#west == West::None && self.r#east == East::Tall { return 10371; }
        if self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::Low { return 10186; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#up == true { return 10202; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::None && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == true { return 10142; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == false { return 10427; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == false && self.r#north == North::Tall { return 10208; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true { return 10130; }
        if self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::Low && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false { return 10313; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Tall { return 10397; }
        if self.r#south == South::Tall && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Tall { return 10210; }
        if self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::None && self.r#north == North::None && self.r#west == West::Low { return 10222; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::None && self.r#up == true { return 10120; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::None && self.r#south == South::None && self.r#up == false && self.r#west == West::Low { return 10114; }
        if self.r#east == East::Low && self.r#west == West::Tall && self.r#up == false && self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == false { return 10247; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Low && self.r#up == false && self.r#west == West::None { return 10257; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#up == true { return 10262; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#east == East::Tall && self.r#west == West::Low { return 10357; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true { return 10274; }
        if self.r#up == false && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == false { return 10413; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true && self.r#east == East::Low && self.r#south == South::Low { return 10265; }
        if self.r#west == West::Tall && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == false { return 10391; }
        if self.r#east == East::Tall && self.r#up == false && self.r#north == North::Tall && self.r#west == West::Tall && self.r#waterlogged == true && self.r#south == South::Tall { return 10424; }
        if self.r#north == North::None && self.r#east == East::None && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall { return 10127; }
        if self.r#up == true && self.r#north == North::None && self.r#east == East::None && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::None { return 10107; }
        if self.r#up == true && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 10166; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#up == false && self.r#north == North::Low && self.r#west == West::Tall && self.r#east == East::Tall { return 10379; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == true && self.r#north == North::Low && self.r#east == East::Tall && self.r#west == West::Low { return 10384; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Low && self.r#up == true { return 10285; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::None { return 10321; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall { return 10238; }
        if self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == true && self.r#north == North::None && self.r#waterlogged == true { return 10333; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#east == East::Tall { return 10350; }
        if self.r#north == North::Low && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low { return 10256; }
        if self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::None && self.r#west == West::Tall { return 10163; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Low && self.r#east == East::Low && self.r#up == true { return 10309; }
        if self.r#up == false && self.r#east == East::None && self.r#north == North::None && self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == true { return 10110; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::None && self.r#north == North::Low { return 10258; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false { return 10330; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false { return 10339; }
        if self.r#up == false && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::None && self.r#waterlogged == false { return 10197; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == false { return 10423; }
        if self.r#up == false && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Tall { return 10341; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#north == North::Low { return 10164; }
        if self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == false && self.r#north == North::Low { return 10151; }
        if self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::None && self.r#north == North::None { return 10214; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#north == North::Low && self.r#up == true && self.r#west == West::None && self.r#waterlogged == true { return 10152; }
        if self.r#east == East::None && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == true { return 10188; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 10334; }
        if self.r#up == true && self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Tall && self.r#east == East::Tall { return 10373; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::Low && self.r#up == true { return 10382; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Low { return 10170; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 10146; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Tall { return 10417; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#up == true { return 10237; }
        if self.r#south == South::Tall && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false { return 10276; }
        if self.r#north == North::None && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall { return 10346; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::None && self.r#east == East::Tall { return 10395; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#west == West::Low { return 10264; }
        if self.r#south == South::Low && self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Tall { return 10411; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#north == North::Low && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false { return 10169; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#up == true && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Low { return 10236; }
        if self.r#south == South::None && self.r#up == false && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::None { return 10113; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true { return 10178; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#up == false && self.r#east == East::None && self.r#north == North::None { return 10139; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#east == East::None && self.r#north == North::Tall { return 10182; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#up == true && self.r#east == East::None && self.r#north == North::Tall && self.r#west == West::Tall { return 10181; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None && self.r#west == West::None && self.r#waterlogged == true && self.r#up == false { return 10254; }
        if self.r#east == East::None && self.r#west == West::Low && self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::None && self.r#up == true { return 10105; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#up == true && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Low { return 10405; }
        if self.r#up == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Tall { return 10277; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#up == false && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::Low { return 10374; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::None && self.r#up == true { return 10190; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true { return 10279; }
        if self.r#north == North::Tall && self.r#west == West::None && self.r#up == true && self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::None { return 10191; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 10416; }
        if self.r#west == West::Low && self.r#up == false && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == false { return 10306; }
        if self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#west == West::Tall { return 10364; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == true { return 10216; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true { return 10172; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#west == West::Low && self.r#up == true && self.r#south == South::None && self.r#waterlogged == false { return 10324; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#up == false { return 10291; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall { return 10199; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::None && self.r#up == true && self.r#north == North::None && self.r#west == West::None { return 10215; }
        if self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Tall { return 10388; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::Low && self.r#up == true && self.r#west == West::Low { return 10228; }
        if self.r#west == West::Low && self.r#up == false && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == false { return 10162; }
        if self.r#east == East::Tall && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#up == true && self.r#north == North::Low { return 10368; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low { return 10363; }
        if self.r#east == East::Low && self.r#up == false && self.r#south == South::None && self.r#west == West::None && self.r#north == North::Tall && self.r#waterlogged == false { return 10293; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 10118; }
        if self.r#waterlogged == true && self.r#up == false && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::Low && self.r#west == West::None { return 10158; }
        if self.r#up == true && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall { return 10217; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#up == false && self.r#west == West::Tall && self.r#north == North::None && self.r#waterlogged == true { return 10124; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 10412; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::None && self.r#up == false { return 10246; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::Low && self.r#up == true { return 10301; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == true && self.r#west == West::None { return 10419; }
        if self.r#east == East::None && self.r#west == West::None && self.r#south == South::Low && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == false { return 10155; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#up == true && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Low { return 10249; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None { return 10248; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Low { return 10243; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 10272; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::None && self.r#west == West::None && self.r#up == false { return 10329; }
        if self.r#waterlogged == false && self.r#up == false && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Low && self.r#east == East::None { return 10173; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 10349; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true { return 10220; }
        if self.r#north == North::None && self.r#up == true && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Tall { return 10121; }
        if self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false { return 10290; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall && self.r#east == East::None { return 10157; }
        if self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Low && self.r#west == West::Tall && self.r#north == North::Tall { return 10319; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::None && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true { return 10327; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 10362; }
        if self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false { return 10390; }
        if self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::None && self.r#south == South::None { return 10140; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::None { return 10145; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Low && self.r#east == East::Low && self.r#up == false { return 10269; }
        if self.r#east == East::Low && self.r#up == true && self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::Low { return 10263; }
        if self.r#waterlogged == false && self.r#up == false && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Tall { return 10355; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Low { return 10278; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::None && self.r#up == true && self.r#north == North::None { return 10335; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#east == East::Low { return 10234; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Low && self.r#up == true && self.r#west == West::Tall { return 10409; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::None && self.r#up == false { return 10219; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None { return 10123; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#up == false && self.r#south == South::Tall { return 10171; }
        if self.r#south == South::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Low { return 10160; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false { return 10378; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None && self.r#east == East::Low && self.r#up == true && self.r#south == South::Low { return 10225; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Low && self.r#up == false && self.r#north == North::None { return 10231; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::None && self.r#up == true && self.r#west == West::Low && self.r#south == South::None { return 10213; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#up == true && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == true { return 10224; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#up == true && self.r#south == South::Tall && self.r#west == West::None { return 10347; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::Low { return 10175; }
        if self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#west == West::Low { return 10195; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::Tall && self.r#up == true { return 10205; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Tall { return 10310; }
        if self.r#west == West::Low && self.r#south == South::None && self.r#north == North::Tall && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true { return 10393; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == false && self.r#south == South::None && self.r#west == West::Low && self.r#north == North::Tall { return 10402; }
        if self.r#north == North::Low && self.r#west == West::Low && self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::None { return 10366; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::None && self.r#east == East::Tall { return 10383; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 10250; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall { return 10298; }
        if self.r#west == West::Low && self.r#up == false && self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#north == North::None { return 10138; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None { return 10185; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#up == false && self.r#west == West::None && self.r#east == East::Tall && self.r#waterlogged == false { return 10389; }
        if self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Low && self.r#west == West::Low { return 10270; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::Tall && self.r#east == East::None && self.r#up == true && self.r#south == South::Low { return 10154; }
        if self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::None { return 10367; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == true { return 10201; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#up == true { return 10392; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == true { return 10396; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10153 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 10314 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 10358 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 10259 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 10325 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 10345 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 10365 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 10399 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 10421 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 10125 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 10287 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 10331 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 10343 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10401 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 10245 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 10242 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10266 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 10410 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 10147 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10326 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 10271 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 10351 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 10156 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 10305 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10176 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 10183 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 10104 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 10148 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 10177 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 10144 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 10159 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10179 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 10198 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 10251 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 10286 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 10304 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 10420 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 10223 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 10299 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 10165 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 10312 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 10415 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 10240 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 10340 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 10200 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 10194 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 10108 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 10275 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 10294 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 10302 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 10115 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 10184 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10204 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 10295 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 10425 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 10354 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 10376 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 10289 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 10414 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 10211 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 10180 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 10303 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10292 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 10369 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 10328 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 10386 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 10226 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10209 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 10385 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 10283 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 10137 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 10418 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 10187 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10316 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 10342 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 10356 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 10370 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 10297 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 10106 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10387 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 10296 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 10426 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 10227 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 10111 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 10260 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 10119 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 10189 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 10422 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 10149 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 10315 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 10136 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 10282 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 10112 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10253 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 10323 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 10255 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 10332 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 10128 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 10221 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 10134 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10273 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 10288 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 10268 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10193 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 10218 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#west: West::None,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10336 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 10406 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 10117 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 10352 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 10317 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 10307 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 10377 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 10344 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 10353 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 10150 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 10280 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 10203 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10122 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 10141 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 10230 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 10394 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10241 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10116 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 10380 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10403 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 10407 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 10161 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 10244 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 10338 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 10408 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 10132 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 10233 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 10281 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 10206 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10322 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10261 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 10235 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 10192 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 10300 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10131 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 10239 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 10308 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 10129 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 10359 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 10135 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 10167 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 10267 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#up: false,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 10126 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 10109 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 10229 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 10348 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 10372 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10320 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 10381 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10318 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10252 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10311 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10360 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 10398 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 10337 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 10404 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 10207 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 10133 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 10232 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10284 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 10361 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 10168 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 10212 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 10143 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10375 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 10400 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 10174 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 10196 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 10371 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 10186 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 10202 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 10142 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10427 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 10208 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 10130 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 10313 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10397 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 10210 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 10222 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 10120 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 10114 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 10247 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10257 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 10262 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 10357 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 10274 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 10413 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10265 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 10391 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 10424 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 10127 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10107 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 10166 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10379 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 10384 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 10285 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 10321 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 10238 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 10333 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10350 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 10256 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 10163 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 10309 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 10110 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10258 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 10330 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 10339 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 10197 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10423 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 10341 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 10164 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 10151 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 10214 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 10152 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10188 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 10334 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10373 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 10382 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 10170 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 10146 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 10417 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 10237 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 10276 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 10346 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 10395 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 10264 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 10411 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 10169 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10236 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 10113 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 10178 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 10139 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 10182 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 10181 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 10254 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 10105 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 10405 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 10277 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 10374 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 10190 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 10279 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10191 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 10416 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 10306 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10364 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 10216 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 10172 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10324 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10291 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 10199 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10215 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 10388 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10228 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 10162 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10368 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 10363 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 10293 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10118 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10158 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#up: false,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 10217 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10124 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10412 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10246 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 10301 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 10419 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 10155 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10249 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 10248 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 10243 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 10272 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 10329 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 10173 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 10349 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10220 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10121 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 10290 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 10157 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 10319 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 10327 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10362 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 10390 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 10140 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 10145 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 10269 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 10263 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 10355 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 10278 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 10335 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 10234 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 10409 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10219 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 10123 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 10171 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 10160 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 10378 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 10225 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 10231 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 10213 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 10224 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10347 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 10175 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 10195 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 10205 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 10310 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10393 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 10402 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 10366 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 10383 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 10250 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10298 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10138 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 10185 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 10389 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10270 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 10154 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 10367 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 10201 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 10392 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 10396 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        return None;
    }
}

