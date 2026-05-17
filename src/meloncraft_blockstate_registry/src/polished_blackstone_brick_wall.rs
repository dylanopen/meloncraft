use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedBlackstoneBrickWall {
    pub r#west: West,
    pub r#east: East,
    pub r#south: South,
    pub up: bool,
    pub waterlogged: bool,
    pub r#north: North,
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

impl BlockState for PolishedBlackstoneBrickWall {
    fn to_id(&self) -> i32 {
        if self.r#south == South::None && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Low { return 22283; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::None && self.r#up == true { return 22310; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#up == true && self.r#south == South::Tall && self.r#west == West::Tall && self.r#waterlogged == true { return 22192; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#west == West::Tall && self.r#waterlogged == true && self.r#up == true && self.r#east == East::Low { return 22288; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#up == false { return 22136; }
        if self.r#up == true && self.r#south == South::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Low { return 22178; }
        if self.r#east == East::Tall && self.r#up == false && self.r#south == South::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 22402; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#up == true && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == true { return 22155; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#west == West::None && self.r#south == South::Low { return 22286; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall { return 22236; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::None && self.r#up == false && self.r#west == West::Tall { return 22273; }
        if self.r#up == false && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Low { return 22293; }
        if self.r#up == true && self.r#north == North::Tall && self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::None { return 22312; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#up == true && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Low { return 22359; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true { return 22292; }
        if self.r#waterlogged == false && self.r#up == false && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Tall { return 22307; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::None && self.r#north == North::None { return 22240; }
        if self.r#east == East::Tall && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::Low { return 22366; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::None && self.r#east == East::None && self.r#up == false { return 22235; }
        if self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Tall && self.r#east == East::None { return 22201; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#up == true && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall { return 22322; }
        if self.r#south == South::None && self.r#west == West::None && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall { return 22424; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#west == West::None { return 22421; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == false && self.r#north == North::None && self.r#south == South::None && self.r#west == West::Tall { return 22357; }
        if self.r#east == East::None && self.r#west == West::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#south == South::Tall { return 22194; }
        if self.r#south == South::None && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low { return 22386; }
        if self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == true { return 22385; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::None { return 22180; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true { return 22218; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::None { return 22257; }
        if self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Low { return 22330; }
        if self.r#north == North::None && self.r#east == East::None && self.r#up == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == true { return 22156; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::None && self.r#up == false && self.r#west == West::Tall { return 22369; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#up == false && self.r#west == West::None { return 22388; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::None { return 22316; }
        if self.r#up == true && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Tall && self.r#waterlogged == false { return 22435; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == false && self.r#up == true { return 22170; }
        if self.r#up == false && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::None { return 22141; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == true && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Low { return 22361; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == true && self.r#west == West::None { return 22370; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true { return 22204; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == false && self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::Tall { return 22309; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None { return 22242; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#west == West::Tall && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false { return 22267; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true && self.r#south == South::None { return 22384; }
        if self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#south == South::None { return 22420; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::None && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true { return 22208; }
        if self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Low { return 22399; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#east == East::Tall { return 22381; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false { return 22422; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#up == true && self.r#east == East::Tall { return 22419; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == true { return 22436; }
        if self.r#up == true && self.r#south == South::None && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low { return 22134; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::Tall { return 22252; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 22248; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#west == West::None && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true { return 22274; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == true { return 22313; }
        if self.r#waterlogged == false && self.r#up == true && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#west == West::None { return 22217; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Low { return 22256; }
        if self.r#east == East::Low && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low { return 22325; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::None && self.r#up == false && self.r#east == East::Tall && self.r#west == West::Low { return 22353; }
        if self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false { return 22404; }
        if self.r#south == South::Low && self.r#west == West::None && self.r#north == North::Low && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false { return 22181; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 22258; }
        if self.r#up == false && self.r#north == North::None && self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Low { return 22244; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false { return 22284; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low { return 22317; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#north == North::None && self.r#west == West::None { return 22355; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true { return 22276; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low && self.r#up == true { return 22289; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::None { return 22151; }
        if self.r#up == true && self.r#south == South::Tall && self.r#north == North::Low && self.r#west == West::None && self.r#east == East::Tall && self.r#waterlogged == true { return 22406; }
        if self.r#up == false && self.r#west == West::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall { return 22380; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#up == true && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low { return 22146; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::None && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == false { return 22213; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Tall && self.r#up == true { return 22206; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low { return 22338; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#up == false && self.r#south == South::Tall && self.r#north == North::Low { return 22196; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Tall && self.r#north == North::None && self.r#up == false { return 22162; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::None && self.r#up == false { return 22174; }
        if self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Low && self.r#north == North::Low { return 22389; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true { return 22443; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::None { return 22154; }
        if self.r#north == North::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None { return 22383; }
        if self.r#up == true && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::Low { return 22395; }
        if self.r#east == East::None && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Low && self.r#up == true { return 22131; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None { return 22232; }
        if self.r#south == South::None && self.r#west == West::Tall && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Low { return 22393; }
        if self.r#south == South::None && self.r#up == true && self.r#north == North::None && self.r#west == West::None && self.r#east == East::Tall && self.r#waterlogged == false { return 22349; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false { return 22302; }
        if self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::Low { return 22428; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#up == true && self.r#south == South::None && self.r#west == West::None { return 22238; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true { return 22231; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::Low && self.r#up == false && self.r#west == West::Tall && self.r#south == South::Low { return 22297; }
        if self.r#north == North::Tall && self.r#up == true && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None { return 22433; }
        if self.r#up == false && self.r#west == West::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::None { return 22245; }
        if self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Tall { return 22337; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::None && self.r#west == West::Low && self.r#up == false { return 22188; }
        if self.r#up == true && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall { return 22255; }
        if self.r#west == West::Low && self.r#up == true && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == true { return 22191; }
        if self.r#north == North::None && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall { return 22165; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::None { return 22350; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::None && self.r#east == East::Tall && self.r#up == true { return 22445; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false { return 22271; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Low && self.r#up == true && self.r#south == South::Low { return 22287; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#east == East::None && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true { return 22148; }
        if self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::None && self.r#up == false && self.r#south == South::Low && self.r#east == East::Low { return 22261; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#up == false && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Tall { return 22329; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#up == false && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == true { return 22305; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Tall { return 22216; }
        if self.r#north == North::None && self.r#west == West::Low && self.r#east == East::None && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true { return 22137; }
        if self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Low { return 22327; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#up == true && self.r#east == East::None { return 22145; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 22270; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Tall { return 22343; }
        if self.r#north == North::None && self.r#west == West::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Low && self.r#up == true { return 22362; }
        if self.r#up == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::None { return 22365; }
        if self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low && self.r#east == East::None { return 22187; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::None && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true { return 22425; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Low && self.r#up == true { return 22335; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::None { return 22203; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Low && self.r#up == false && self.r#west == West::Tall && self.r#north == North::None { return 22150; }
        if self.r#east == East::Low && self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false { return 22249; }
        if self.r#east == East::None && self.r#up == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Tall { return 22234; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::None && self.r#up == false { return 22268; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low { return 22449; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#up == true && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::None { return 22277; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true { return 22263; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Low && self.r#up == false { return 22342; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#up == false && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Low { return 22368; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false && self.r#south == South::Low && self.r#west == West::None { return 22331; }
        if self.r#up == true && self.r#north == North::Low && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == false { return 22397; }
        if self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::None && self.r#up == false { return 22390; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#up == false { return 22185; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#up == true && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall { return 22351; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true && self.r#north == North::Tall { return 22228; }
        if self.r#south == South::None && self.r#up == true && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Low { return 22239; }
        if self.r#west == West::Low && self.r#north == North::None && self.r#east == East::Tall && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == true { return 22371; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#up == true && self.r#west == West::None { return 22358; }
        if self.r#waterlogged == false && self.r#up == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::None { return 22387; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Tall && self.r#east == East::Tall && self.r#up == true && self.r#south == South::None { return 22348; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None { return 22168; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == true && self.r#east == East::None { return 22182; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::None && self.r#east == East::None && self.r#up == false { return 22210; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true && self.r#south == South::None { return 22318; }
        if self.r#waterlogged == false && self.r#up == true && self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::Tall && self.r#west == West::Tall { return 22423; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == true && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::Tall { return 22442; }
        if self.r#north == North::Tall && self.r#west == West::None && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Tall { return 22451; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::None { return 22175; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::None && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true { return 22132; }
        if self.r#up == true && self.r#south == South::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None { return 22202; }
        if self.r#up == false && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None { return 22259; }
        if self.r#up == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::Tall { return 22336; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Tall { return 22265; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Low { return 22251; }
        if self.r#south == South::Tall && self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::Tall && self.r#up == true { return 22375; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#up == true && self.r#south == South::Tall { return 22446; }
        if self.r#up == false && self.r#east == East::None && self.r#north == North::Low && self.r#west == West::Low && self.r#south == South::None && self.r#waterlogged == false { return 22176; }
        if self.r#waterlogged == true && self.r#up == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Tall && self.r#west == West::None { return 22412; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low { return 22183; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true { return 22328; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true { return 22341; }
        if self.r#up == false && self.r#west == West::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Low { return 22272; }
        if self.r#up == false && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Low { return 22152; }
        if self.r#west == West::None && self.r#up == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::None { return 22214; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#up == true && self.r#east == East::None && self.r#north == North::None && self.r#south == South::Tall { return 22158; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Tall && self.r#west == West::Tall && self.r#up == true { return 22432; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false { return 22219; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::Tall { return 22372; }
        if self.r#west == West::None && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == false { return 22379; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#west == West::Low && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Low { return 22398; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None && self.r#north == North::None && self.r#up == false { return 22247; }
        if self.r#south == South::Tall && self.r#up == false && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low { return 22161; }
        if self.r#up == false && self.r#north == North::Low && self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::None && self.r#waterlogged == false { return 22391; }
        if self.r#west == West::Low && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::Low { return 22407; }
        if self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::None { return 22418; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::None && self.r#west == West::None && self.r#north == North::None && self.r#up == true { return 22241; }
        if self.r#up == true && self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::Low && self.r#east == East::None && self.r#west == West::Low { return 22179; }
        if self.r#north == North::Tall && self.r#up == true && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low { return 22215; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Tall && self.r#up == true && self.r#east == East::None { return 22171; }
        if self.r#west == West::Low && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Low { return 22437; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false && self.r#east == East::Tall && self.r#west == West::Tall && self.r#waterlogged == false { return 22441; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#up == true && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == false { return 22253; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false { return 22429; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None && self.r#up == true { return 22279; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::None { return 22282; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::None && self.r#up == true && self.r#south == South::Low { return 22142; }
        if self.r#south == South::None && self.r#west == West::None && self.r#up == false && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Tall { return 22211; }
        if self.r#east == East::None && self.r#west == West::Low && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::Tall { return 22224; }
        if self.r#up == true && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Low && self.r#west == West::Low && self.r#waterlogged == false { return 22290; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#up == false && self.r#west == West::Tall && self.r#north == North::Low { return 22294; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#up == false && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == false { return 22200; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::None && self.r#up == false && self.r#west == West::Low { return 22209; }
        if self.r#up == false && self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::None && self.r#east == East::None && self.r#west == West::Tall { return 22138; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#up == false && self.r#north == North::Tall && self.r#west == West::None && self.r#south == South::Low { return 22223; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#up == true { return 22301; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false && self.r#west == West::None && self.r#waterlogged == true { return 22352; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#up == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == true { return 22198; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::Tall { return 22374; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::None && self.r#up == true { return 22159; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::Low && self.r#up == true { return 22298; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#west == West::None { return 22382; }
        if self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == true { return 22197; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#up == true && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false { return 22326; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#east == East::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall { return 22339; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#up == true && self.r#west == West::Low { return 22347; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Low { return 22306; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Tall && self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == true { return 22396; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true { return 22264; }
        if self.r#waterlogged == true && self.r#up == false && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::None && self.r#west == West::Low { return 22149; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#up == true && self.r#west == West::None && self.r#south == South::None && self.r#north == North::Low { return 22166; }
        if self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::None && self.r#north == North::Tall { return 22448; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Low && self.r#up == false && self.r#west == West::Low { return 22308; }
        if self.r#north == North::Low && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::Low { return 22189; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::None && self.r#north == North::Tall { return 22221; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 22394; }
        if self.r#west == West::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == false { return 22405; }
        if self.r#west == West::Tall && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Low { return 22408; }
        if self.r#east == East::Tall && self.r#up == false && self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Tall { return 22427; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::None && self.r#up == false { return 22172; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::None && self.r#up == false { return 22280; }
        if self.r#south == South::None && self.r#up == true && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low { return 22311; }
        if self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Tall && self.r#west == West::Low && self.r#waterlogged == true && self.r#up == true { return 22323; }
        if self.r#up == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::None { return 22360; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#west == West::Tall && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true { return 22222; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::Tall { return 22345; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::None && self.r#east == East::Low && self.r#waterlogged == false && self.r#up == true { return 22314; }
        if self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false { return 22434; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Low && self.r#up == false && self.r#north == North::Low && self.r#south == South::Low { return 22296; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 22246; }
        if self.r#south == South::None && self.r#east == East::None && self.r#west == West::Tall && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false { return 22177; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::Low && self.r#up == true && self.r#west == West::None { return 22193; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Tall { return 22378; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == true && self.r#south == South::Tall { return 22299; }
        if self.r#west == West::None && self.r#north == North::None && self.r#waterlogged == false && self.r#up == true && self.r#south == South::Tall && self.r#east == East::None { return 22157; }
        if self.r#east == East::None && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == true { return 22227; }
        if self.r#up == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Tall && self.r#waterlogged == false { return 22453; }
        if self.r#up == false && self.r#west == West::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::Tall { return 22376; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == true && self.r#north == North::Tall { return 22220; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None && self.r#up == true && self.r#north == North::Tall { return 22205; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::None { return 22163; }
        if self.r#up == true && self.r#south == South::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Tall { return 22230; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None && self.r#north == North::Low && self.r#up == true && self.r#south == South::None { return 22169; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::None { return 22275; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#up == false && self.r#south == South::Tall && self.r#east == East::None && self.r#west == West::Low { return 22164; }
        if self.r#north == North::Tall && self.r#up == false && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Low { return 22320; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::None { return 22226; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::None { return 22363; }
        if self.r#south == South::None && self.r#north == North::None && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#up == false { return 22356; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Low { return 22400; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false { return 22410; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Tall { return 22340; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::None { return 22315; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#up == false && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::None { return 22364; }
        if self.r#up == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Tall { return 22321; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::None && self.r#up == true { return 22229; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low { return 22254; }
        if self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true { return 22444; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#up == true && self.r#south == South::None && self.r#north == North::Tall && self.r#waterlogged == false { return 22207; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall { return 22285; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 22450; }
        if self.r#west == West::None && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Tall && self.r#east == East::None { return 22190; }
        if self.r#south == South::Low && self.r#up == false && self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Low { return 22260; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::None && self.r#up == false && self.r#west == West::Low && self.r#south == South::Tall { return 22269; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#up == false { return 22415; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false && self.r#south == South::Low { return 22332; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == false { return 22416; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Low && self.r#east == East::None && self.r#up == true && self.r#west == West::Tall { return 22147; }
        if self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::None && self.r#west == West::None { return 22160; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == false && self.r#north == North::Low && self.r#east == East::Tall { return 22413; }
        if self.r#north == North::None && self.r#east == East::None && self.r#south == South::Low && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == true { return 22143; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#up == false && self.r#east == East::None && self.r#west == West::Low && self.r#south == South::None { return 22140; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#up == false && self.r#west == West::None && self.r#east == East::None { return 22184; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::None && self.r#up == true { return 22262; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#up == true && self.r#south == South::Tall && self.r#north == North::Low && self.r#west == West::Tall { return 22300; }
        if self.r#up == true && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::None { return 22346; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#up == false && self.r#south == South::Tall && self.r#west == West::Low { return 22377; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::Low && self.r#up == false { return 22392; }
        if self.r#up == true && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None { return 22243; }
        if self.r#up == true && self.r#south == South::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None { return 22373; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#north == North::None && self.r#up == true { return 22133; }
        if self.r#up == false && self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::Low { return 22333; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 22186; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#waterlogged == false { return 22411; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Tall { return 22291; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Tall && self.r#west == West::Tall && self.r#up == false && self.r#north == North::Low { return 22417; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low { return 22430; }
        if self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true { return 22431; }
        if self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Tall { return 22447; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#up == false { return 22403; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::None && self.r#up == false && self.r#south == South::Low && self.r#west == West::None { return 22367; }
        if self.r#north == North::Low && self.r#up == true && self.r#east == East::None && self.r#south == South::Tall && self.r#west == West::Tall && self.r#waterlogged == false { return 22195; }
        if self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Low && self.r#south == South::Tall { return 22452; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#up == false && self.r#west == West::None { return 22199; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#up == true && self.r#west == West::Tall { return 22303; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None { return 22319; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Low && self.r#up == false && self.r#west == West::None { return 22295; }
        if self.r#up == false && self.r#south == South::None && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall { return 22354; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::Low && self.r#up == false && self.r#west == West::Tall { return 22225; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false && self.r#south == South::Tall { return 22237; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == true { return 22281; }
        if self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Low && self.r#up == false { return 22173; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::None && self.r#up == false && self.r#north == North::Tall { return 22233; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false { return 22278; }
        if self.r#north == North::None && self.r#up == true && self.r#east == East::None && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::None { return 22130; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::Tall && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == true { return 22334; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#west == West::Low { return 22167; }
        if self.r#up == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Low { return 22414; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::Tall && self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == false { return 22426; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#up == false && self.r#north == North::Tall && self.r#south == South::Low { return 22439; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true { return 22250; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#east == East::Low { return 22344; }
        if self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::None && self.r#west == West::Tall && self.r#up == true { return 22144; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::Tall { return 22324; }
        if self.r#east == East::Tall && self.r#up == false && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == true { return 22401; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Tall { return 22266; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == false && self.r#east == East::Tall && self.r#west == West::Tall { return 22438; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Tall && self.r#south == South::None && self.r#up == true { return 22135; }
        if self.r#up == false && self.r#west == West::None && self.r#east == East::None && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::None { return 22139; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == true { return 22409; }
        if self.r#west == West::Low && self.r#up == false && self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Tall { return 22212; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#up == false { return 22304; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Low { return 22440; }
        if self.r#south == South::Low && self.r#up == false && self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Tall { return 22153; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22283 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 22310 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 22192 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22288 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 22136 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 22178 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 22402 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22155 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 22286 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 22236 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 22273 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 22293 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 22312 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 22359 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 22292 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22307 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 22240 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 22366 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 22235 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 22201 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 22322 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 22424 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 22421 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 22357 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 22194 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 22386 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 22385 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 22180 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 22218 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 22257 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 22330 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 22156 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22369 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 22388 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 22316 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22435 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22170 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 22141 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 22361 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 22370 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 22204 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22309 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 22242 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 22267 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22384 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 22420 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 22208 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22399 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 22381 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 22422 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 22419 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 22436 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 22134 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 22252 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 22248 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 22274 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22313 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 22217 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 22256 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 22325 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 22353 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 22404 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 22181 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22258 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22244 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 22284 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 22317 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 22355 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 22276 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22289 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 22151 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 22406 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22380 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 22146 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 22213 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22206 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 22338 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 22196 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 22162 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 22174 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 22389 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 22443 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 22154 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 22383 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 22395 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 22131 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 22232 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22393 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 22349 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22302 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22428 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 22238 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 22231 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 22297 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 22433 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 22245 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 22337 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 22188 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 22255 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 22191 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22165 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 22350 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 22445 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 22271 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 22287 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 22148 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22261 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 22329 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 22305 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22216 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 22137 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 22327 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 22145 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 22270 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22343 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 22362 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 22365 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 22187 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 22425 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22335 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 22203 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 22150 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 22249 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22234 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 22268 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 22449 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 22277 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 22263 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22342 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 22368 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 22331 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 22397 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 22390 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 22185 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 22351 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 22228 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 22239 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 22371 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22358 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 22387 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 22348 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 22168 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 22182 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 22210 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 22318 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 22423 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 22442 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 22451 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#west: West::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 22175 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 22132 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22202 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 22259 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 22336 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 22265 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 22251 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 22375 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 22446 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 22176 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22412 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 22183 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 22328 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22341 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22272 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 22152 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 22214 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 22158 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 22432 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 22219 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22372 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 22379 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22398 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 22247 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 22161 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 22391 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 22407 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 22418 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 22241 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 22179 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 22215 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 22171 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 22437 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 22441 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22253 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 22429 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22279 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 22282 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 22142 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 22211 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 22224 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 22290 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 22294 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 22200 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22209 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 22138 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 22223 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 22301 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 22352 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 22198 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22374 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 22159 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 22298 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 22382 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 22197 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 22326 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 22339 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22347 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 22306 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 22396 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22264 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 22149 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 22166 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 22448 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 22308 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 22189 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 22221 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 22394 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22405 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22408 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 22427 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 22172 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 22280 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 22311 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 22323 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 22360 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 22222 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22345 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 22314 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 22434 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22296 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 22246 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22177 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22193 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 22378 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 22299 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 22157 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 22227 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 22453 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22376 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 22220 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 22205 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 22163 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 22230 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 22169 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 22275 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 22164 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 22320 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 22226 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22363 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 22356 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 22400 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 22410 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22340 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 22315 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 22364 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22321 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 22229 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 22254 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 22444 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 22207 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22285 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 22450 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22190 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 22260 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 22269 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 22415 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 22332 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 22416 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 22147 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22160 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 22413 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 22143 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 22140 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 22184 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 22262 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 22300 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 22346 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22377 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 22392 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 22243 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 22373 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 22133 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 22333 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 22186 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22411 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 22291 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 22417 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 22430 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 22431 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22447 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 22403 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 22367 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 22195 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22452 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 22199 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 22303 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 22319 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 22295 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 22354 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 22225 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 22237 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 22281 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22173 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 22233 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 22278 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 22130 {
            return Some(PolishedBlackstoneBrickWall {
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 22334 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 22167 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 22414 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 22426 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 22439 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 22250 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 22344 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 22144 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 22324 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 22401 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 22266 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 22438 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 22135 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 22139 {
            return Some(PolishedBlackstoneBrickWall {
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 22409 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 22212 {
            return Some(PolishedBlackstoneBrickWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 22304 {
            return Some(PolishedBlackstoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 22440 {
            return Some(PolishedBlackstoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 22153 {
            return Some(PolishedBlackstoneBrickWall {
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        return None;
    }
}

