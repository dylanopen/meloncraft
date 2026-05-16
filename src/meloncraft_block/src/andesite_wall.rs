use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AndesiteWall {
    pub waterlogged: bool,
    pub r#south: South,
    pub r#east: East,
    pub r#west: West,
    pub up: bool,
    pub r#north: North,
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

impl BlockState for AndesiteWall {
    fn to_id(&self) -> i32 {
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::None { return 18898; }
        if self.r#up == true && self.r#west == West::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::Tall { return 19127; }
        if self.r#north == North::Low && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::None { return 19141; }
        if self.r#up == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#east == East::Low && self.r#west == West::Low && self.r#waterlogged == true { return 19095; }
        if self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::None { return 19133; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#up == true && self.r#west == West::None && self.r#south == South::Tall { return 19160; }
        if self.r#up == true && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::None { return 18993; }
        if self.r#up == true && self.r#west == West::Low && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Tall && self.r#east == East::None { return 18945; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true && self.r#east == East::Tall { return 19129; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#up == true { return 19187; }
        if self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == true && self.r#east == East::None { return 18971; }
        if self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Low { return 19042; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None && self.r#east == East::None && self.r#south == South::Tall && self.r#up == false { return 18915; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#south == South::None && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == false { return 18931; }
        if self.r#south == South::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Low { return 18975; }
        if self.r#west == West::Low && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Low { return 19047; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Tall { return 18943; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == true && self.r#south == South::Low { return 19117; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::None && self.r#up == true { return 18958; }
        if self.r#up == true && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Tall { return 18913; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#south == South::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == false { return 19070; }
        if self.r#up == true && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::Tall { return 19140; }
        if self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#south == South::Low { return 19159; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == true && self.r#north == North::Low { return 19044; }
        if self.r#up == false && self.r#south == South::Low && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == false { return 19015; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true { return 18951; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#up == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == false { return 19098; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::Low && self.r#west == West::None && self.r#up == true && self.r#south == South::None { return 19064; }
        if self.r#up == false && self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::Tall { return 19191; }
        if self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Tall { return 19207; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::None && self.r#east == East::None && self.r#up == true { return 18932; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::Tall && self.r#up == true && self.r#north == North::None { return 19125; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true { return 18952; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::Tall { return 19164; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == true && self.r#east == East::Tall && self.r#west == West::Tall && self.r#north == North::Low { return 19165; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#up == true && self.r#south == South::Low && self.r#west == West::Tall && self.r#north == North::None { return 18901; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Low && self.r#up == false && self.r#north == North::Low { return 19059; }
        if self.r#east == East::None && self.r#up == true && self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Tall { return 18911; }
        if self.r#south == South::Low && self.r#west == West::None && self.r#east == East::Tall && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == true { return 19184; }
        if self.r#up == false && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Tall && self.r#south == South::Tall { return 19024; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Tall { return 19108; }
        if self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true { return 19114; }
        if self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false { return 18955; }
        if self.r#south == South::Tall && self.r#up == true && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::Low && self.r#waterlogged == true { return 19052; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Low { return 19094; }
        if self.r#south == South::Low && self.r#east == East::Low && self.r#up == true && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low { return 19008; }
        if self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Low && self.r#west == West::Tall { return 19099; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 19148; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None && self.r#east == East::None { return 18959; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == false && self.r#south == South::None && self.r#west == West::Low { return 19179; }
        if self.r#up == true && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == false { return 19151; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#up == false && self.r#south == South::None && self.r#west == West::None { return 19034; }
        if self.r#south == South::Tall && self.r#up == false && self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 19168; }
        if self.r#east == East::None && self.r#south == South::None && self.r#west == West::Tall && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == false { return 18967; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low { return 19085; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Tall { return 18946; }
        if self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Tall { return 19192; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#north == North::None { return 19132; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::None && self.r#up == true && self.r#north == North::Low { return 18924; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == false && self.r#east == East::Tall && self.r#north == North::Tall { return 19190; }
        if self.r#east == East::Tall && self.r#up == false && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::None { return 19193; }
        if self.r#up == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Tall { return 19206; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::None && self.r#west == West::Tall && self.r#up == true { return 18910; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#up == true { return 18920; }
        if self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Tall && self.r#up == false { return 18963; }
        if self.r#west == West::Low && self.r#up == true && self.r#south == South::None && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == false { return 18888; }
        if self.r#east == East::None && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Low { return 18900; }
        if self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#east == East::None { return 18934; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == true && self.r#west == West::None && self.r#south == South::Tall { return 19091; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == false && self.r#west == West::None { return 19082; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall { return 18989; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None { return 18956; }
        if self.r#north == North::Tall && self.r#up == false && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall { return 18987; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low { return 18973; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == true { return 18985; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#up == false && self.r#west == West::None && self.r#south == South::None { return 19001; }
        if self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#up == true { return 19136; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Tall { return 18961; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true { return 19155; }
        if self.r#south == South::Tall && self.r#up == true && self.r#north == North::Low && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low { return 18948; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Tall { return 19171; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Low && self.r#up == true && self.r#north == North::Low && self.r#west == West::Tall { return 19153; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::None && self.r#up == true && self.r#south == South::None && self.r#waterlogged == true { return 18957; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None { return 19017; }
        if self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Tall && self.r#north == North::Low && self.r#south == South::None { return 19030; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#up == true && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Tall { return 19092; }
        if self.r#west == West::Tall && self.r#north == North::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::None && self.r#up == true { return 18925; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#up == false && self.r#west == West::Low { return 18942; }
        if self.r#east == East::Tall && self.r#up == false && self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall { return 19135; }
        if self.r#north == North::Low && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true { return 19137; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#up == true && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == true { return 18970; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true { return 18988; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::Tall && self.r#up == true && self.r#south == South::Tall && self.r#west == West::Tall { return 19126; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::None && self.r#up == false { return 18926; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Low { return 19158; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true { return 19053; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Low && self.r#up == false && self.r#south == South::Tall { return 19062; }
        if self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#east == East::None && self.r#up == true && self.r#west == West::Tall { return 18922; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#up == true && self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == false { return 19021; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false { return 19027; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false { return 19093; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true { return 18976; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == true { return 18994; }
        if self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::None { return 18992; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::None { return 19055; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::None && self.r#south == South::None && self.r#up == false && self.r#east == East::Tall { return 19109; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::None && self.r#up == false { return 19121; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == true && self.r#west == West::None && self.r#north == North::Low { return 19163; }
        if self.r#up == true && self.r#south == South::None && self.r#east == East::None && self.r#north == North::None && self.r#west == West::Tall && self.r#waterlogged == false { return 18889; }
        if self.r#east == East::None && self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#up == false { return 18966; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::None && self.r#up == true && self.r#west == West::Low && self.r#east == East::Tall { return 19101; }
        if self.r#north == North::None && self.r#west == West::None && self.r#south == South::Tall && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false { return 18917; }
        if self.r#south == South::None && self.r#waterlogged == true && self.r#up == false && self.r#east == East::None && self.r#west == West::Low && self.r#north == North::Low { return 18927; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Tall && self.r#up == false { return 19195; }
        if self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::Low { return 19050; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::None { return 19079; }
        if self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Tall { return 19196; }
        if self.r#east == East::Tall && self.r#up == false && self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == true { return 19202; }
        if self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::None && self.r#north == North::Low && self.r#west == West::Low { return 18921; }
        if self.r#east == East::None && self.r#up == true && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == true { return 18933; }
        if self.r#east == East::Low && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false { return 19007; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#up == false && self.r#south == South::Low { return 19012; }
        if self.r#south == South::None && self.r#up == false && self.r#west == West::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Tall { return 19178; }
        if self.r#up == false && self.r#north == North::Tall && self.r#west == West::None && self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == true { return 18986; }
        if self.r#west == West::Low && self.r#north == North::None && self.r#up == false && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false { return 19014; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Tall { return 19039; }
        if self.r#up == false && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == true { return 19071; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#up == false { return 19142; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#waterlogged == false && self.r#up == true && self.r#south == South::Low && self.r#north == North::Low { return 19043; }
        if self.r#west == West::Tall && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low { return 19081; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false { return 19204; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low { return 18937; }
        if self.r#south == South::None && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None { return 19002; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low && self.r#east == East::Low { return 19076; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::Tall { return 19097; }
        if self.r#east == East::None && self.r#up == true && self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::None && self.r#waterlogged == true { return 18886; }
        if self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::None { return 19122; }
        if self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::None && self.r#up == false { return 19180; }
        if self.r#east == East::Low && self.r#west == West::Low && self.r#south == South::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false { return 18996; }
        if self.r#south == South::Low && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == false && self.r#up == false { return 18907; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Tall { return 19147; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#up == false && self.r#east == East::None && self.r#west == West::Tall && self.r#north == North::Tall { return 18979; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Tall { return 19189; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall { return 19123; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false { return 18941; }
        if self.r#west == West::None && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == true { return 18944; }
        if self.r#south == South::None && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::None { return 19037; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Low && self.r#up == false && self.r#north == North::None { return 19134; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::None && self.r#up == false { return 18929; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#east == East::None && self.r#west == West::Low { return 18903; }
        if self.r#south == South::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false && self.r#north == North::Low { return 18939; }
        if self.r#north == North::None && self.r#west == West::None && self.r#east == East::None && self.r#waterlogged == true && self.r#up == true && self.r#south == South::None { return 18884; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall && self.r#south == South::None { return 19069; }
        if self.r#east == East::None && self.r#north == North::None && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Low { return 18897; }
        if self.r#east == East::Tall && self.r#up == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == false { return 19128; }
        if self.r#east == East::Low && self.r#up == true && self.r#south == South::None && self.r#north == North::Low && self.r#west == West::Low && self.r#waterlogged == false { return 19032; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == true { return 19172; }
        if self.r#up == true && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None && self.r#north == North::None && self.r#south == South::None { return 18887; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true { return 19040; }
        if self.r#north == North::None && self.r#up == false && self.r#east == East::None && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall { return 18892; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false { return 18990; }
        if self.r#west == West::Low && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::None { return 19131; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#up == false && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Low { return 19157; }
        if self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Tall { return 19096; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall && self.r#up == false { return 19181; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::None && self.r#up == false && self.r#west == West::None { return 18893; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#up == false && self.r#north == North::None && self.r#south == South::Low { return 19013; }
        if self.r#south == South::None && self.r#north == North::None && self.r#waterlogged == true && self.r#up == false && self.r#west == West::None && self.r#east == East::Low { return 18998; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#up == true && self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::None { return 19068; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == false && self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::Tall { return 19203; }
        if self.r#north == North::None && self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == true { return 18885; }
        if self.r#south == South::Low && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Tall { return 19087; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::None && self.r#up == false && self.r#west == West::None { return 18974; }
        if self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None && self.r#north == North::Low { return 18923; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#up == true && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Low { return 19041; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::None && self.r#up == false && self.r#west == West::Low && self.r#north == North::None { return 18894; }
        if self.r#up == false && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::None { return 19154; }
        if self.r#up == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::None { return 18912; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Tall && self.r#up == true && self.r#east == East::Low { return 19088; }
        if self.r#up == true && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#waterlogged == false { return 19176; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == true && self.r#south == South::None && self.r#north == North::Tall { return 19065; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#waterlogged == false && self.r#up == true && self.r#east == East::Tall && self.r#north == North::Tall { return 19177; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#up == true && self.r#south == South::None { return 18997; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Low { return 19010; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall { return 18991; }
        if self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Tall && self.r#north == North::Low { return 19138; }
        if self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Low { return 19167; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == false && self.r#north == North::None { return 18918; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Low && self.r#up == true && self.r#west == West::None && self.r#north == North::Low { return 19028; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true { return 19188; }
        if self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#east == East::Low { return 19048; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true { return 19000; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == false && self.r#up == false && self.r#west == West::Tall { return 18919; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Tall { return 19199; }
        if self.r#north == North::Low && self.r#up == true && self.r#east == East::Low && self.r#south == South::None && self.r#west == West::Low && self.r#waterlogged == true { return 19029; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#up == false && self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::Low { return 19073; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#waterlogged == true && self.r#up == true && self.r#south == South::Low && self.r#west == West::Low { return 18969; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#waterlogged == false { return 19057; }
        if self.r#west == West::Low && self.r#up == false && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::None { return 18999; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low { return 19143; }
        if self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::None { return 19105; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Tall { return 19086; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Low { return 19186; }
        if self.r#waterlogged == false && self.r#up == false && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Low && self.r#west == West::Tall { return 19051; }
        if self.r#up == false && self.r#south == South::Tall && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Low { return 18954; }
        if self.r#up == true && self.r#west == West::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::Low { return 19005; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#south == South::Tall && self.r#north == North::None && self.r#up == false { return 18914; }
        if self.r#up == false && self.r#west == West::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Tall { return 19072; }
        if self.r#east == East::Low && self.r#up == false && self.r#south == South::Low && self.r#north == North::Tall && self.r#west == West::Low && self.r#waterlogged == true { return 19083; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::None && self.r#south == South::Low { return 18904; }
        if self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == false && self.r#up == true && self.r#east == East::Tall && self.r#south == South::None { return 19103; }
        if self.r#north == North::Tall && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == false { return 19084; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true && self.r#east == East::None && self.r#west == West::Tall && self.r#north == North::Tall { return 18982; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#west == West::Tall { return 18895; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low && self.r#up == false && self.r#north == North::None && self.r#east == East::Low { return 19011; }
        if self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Tall && self.r#north == North::None { return 19111; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true && self.r#east == East::Tall && self.r#west == West::Low && self.r#waterlogged == true { return 19197; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None && self.r#up == true { return 18935; }
        if self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::None && self.r#up == false { return 18953; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#up == true && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == true { return 18980; }
        if self.r#up == false && self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == false { return 19026; }
        if self.r#north == North::Low && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low { return 19036; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == true && self.r#south == South::Low && self.r#west == West::Low { return 19080; }
        if self.r#up == false && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::None { return 19107; }
        if self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false { return 19170; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::None { return 19175; }
        if self.r#north == North::Low && self.r#west == West::Low && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Low { return 19152; }
        if self.r#south == South::None && self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::Tall { return 19174; }
        if self.r#up == false && self.r#east == East::None && self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::None && self.r#west == West::Low { return 18891; }
        if self.r#south == South::None && self.r#north == North::Low && self.r#east == East::Low && self.r#west == West::Low && self.r#waterlogged == false && self.r#up == false { return 19038; }
        if self.r#north == North::None && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::Low { return 18906; }
        if self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::Tall { return 19102; }
        if self.r#up == true && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::Tall && self.r#west == West::None && self.r#south == South::Low { return 19115; }
        if self.r#west == West::None && self.r#up == false && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == true { return 19118; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall { return 19075; }
        if self.r#west == West::Tall && self.r#up == true && self.r#south == South::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Low { return 19150; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true { return 19089; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#east == East::None && self.r#west == West::Tall && self.r#up == false && self.r#south == South::None { return 18928; }
        if self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::None && self.r#west == West::None { return 18977; }
        if self.r#west == West::None && self.r#south == South::None && self.r#north == North::None && self.r#waterlogged == false && self.r#up == true && self.r#east == East::Low { return 18995; }
        if self.r#up == true && self.r#north == North::Low && self.r#west == West::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Low { return 19045; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Tall { return 19060; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Low { return 19056; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#up == true && self.r#north == North::Tall { return 19066; }
        if self.r#south == South::Low && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Low { return 19077; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::None && self.r#up == false { return 19049; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::None { return 19124; }
        if self.r#south == South::None && self.r#west == West::None && self.r#up == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == false { return 19139; }
        if self.r#west == West::None && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::None { return 18965; }
        if self.r#up == false && self.r#north == North::None && self.r#south == South::None && self.r#east == East::Tall && self.r#west == West::Low && self.r#waterlogged == false { return 19110; }
        if self.r#east == East::Tall && self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true { return 19201; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::None && self.r#south == South::Low && self.r#up == false && self.r#west == West::None { return 18902; }
        if self.r#up == false && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::None { return 18916; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None { return 18960; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true { return 19046; }
        if self.r#up == true && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::None { return 18908; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::Low && self.r#up == false && self.r#west == West::Low && self.r#south == South::None { return 18930; }
        if self.r#east == East::None && self.r#up == false && self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall { return 18964; }
        if self.r#west == West::Low && self.r#up == true && self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::Low && self.r#waterlogged == false { return 19020; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::None { return 19173; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#west == West::None && self.r#east == East::Tall { return 19145; }
        if self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::None { return 19100; }
        if self.r#south == South::None && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall && self.r#east == East::Low { return 19074; }
        if self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Low { return 19006; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#up == true && self.r#south == South::None { return 19067; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::Tall && self.r#up == false && self.r#west == West::None && self.r#north == North::None { return 19106; }
        if self.r#up == false && self.r#north == North::Low && self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::Low && self.r#waterlogged == true { return 18940; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::None { return 18968; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low && self.r#east == East::None { return 18936; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::None { return 19058; }
        if self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true { return 19161; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::None { return 18896; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::None { return 18938; }
        if self.r#north == North::Low && self.r#up == true && self.r#east == East::Low && self.r#west == West::Tall && self.r#waterlogged == true && self.r#south == South::Tall { return 19054; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true && self.r#east == East::None && self.r#west == West::Low { return 18981; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#up == false && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Low { return 18950; }
        if self.r#up == true && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::None { return 19112; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low { return 19113; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall { return 18983; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None { return 19022; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == true && self.r#west == West::Low { return 19185; }
        if self.r#up == false && self.r#west == West::None && self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::Tall { return 19130; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall { return 19116; }
        if self.r#west == West::Low && self.r#up == true && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::Low { return 19149; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#up == true && self.r#south == South::Low && self.r#west == West::Tall && self.r#waterlogged == true { return 19078; }
        if self.r#up == false && self.r#south == South::None && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Tall { return 19003; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Low && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false { return 19063; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Low { return 19090; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::Tall { return 19162; }
        if self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#up == true { return 18909; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#west == West::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == true { return 18947; }
        if self.r#west == West::None && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == false { return 19025; }
        if self.r#west == West::None && self.r#south == South::None && self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true { return 18890; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 19033; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low { return 19104; }
        if self.r#waterlogged == true && self.r#up == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::None { return 19144; }
        if self.r#south == South::None && self.r#up == false && self.r#north == North::Low && self.r#east == East::Tall && self.r#west == West::Low && self.r#waterlogged == false { return 19146; }
        if self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Tall { return 19156; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Low { return 19169; }
        if self.r#up == false && self.r#east == East::Tall && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Low { return 19182; }
        if self.r#up == false && self.r#south == South::None && self.r#west == West::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Tall { return 19183; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::Tall && self.r#up == false { return 19194; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Tall { return 19198; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Tall { return 19200; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Tall && self.r#north == North::Low && self.r#up == false && self.r#west == West::None { return 19061; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Low && self.r#up == true && self.r#north == North::None { return 19004; }
        if self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::None && self.r#east == East::None && self.r#north == North::None && self.r#up == false { return 18905; }
        if self.r#south == South::Low && self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::None { return 19120; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None { return 18984; }
        if self.r#west == West::None && self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true { return 18962; }
        if self.r#west == West::None && self.r#east == East::None && self.r#north == North::None && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == false { return 18899; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Low { return 18972; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#up == true && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == false { return 19009; }
        if self.r#north == North::None && self.r#west == West::None && self.r#south == South::Tall && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == true { return 19016; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false { return 19031; }
        if self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::Tall { return 19166; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::Tall { return 18978; }
        if self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false { return 18949; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Tall { return 19019; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#north == North::Low && self.r#east == East::Low && self.r#up == false && self.r#south == South::None { return 19035; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Tall && self.r#up == false && self.r#north == North::Tall { return 19205; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::Low && self.r#west == West::Low && self.r#south == South::Tall && self.r#up == false { return 19023; }
        if self.r#up == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::Low { return 19119; }
        if self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Tall { return 19018; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 18898 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 19127 {
            return Some(AndesiteWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19141 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19095 {
            return Some(AndesiteWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19133 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19160 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18993 {
            return Some(AndesiteWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 18945 {
            return Some(AndesiteWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 19129 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 19187 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 18971 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 19042 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 18915 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 18931 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18975 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 19047 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18943 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19117 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 18958 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18913 {
            return Some(AndesiteWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19070 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 19140 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 19159 {
            return Some(AndesiteWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19044 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 19015 {
            return Some(AndesiteWall {
                r#up: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18951 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19098 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19064 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 19191 {
            return Some(AndesiteWall {
                r#up: false,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 19207 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18932 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 19125 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18952 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19164 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 19165 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18901 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19059 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 18911 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19184 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19024 {
            return Some(AndesiteWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19108 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19114 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18955 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19052 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19094 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 19008 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19099 {
            return Some(AndesiteWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 19148 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18959 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 19179 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 19151 {
            return Some(AndesiteWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19034 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 19168 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18967 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19085 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 18946 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 19192 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 19132 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 18924 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 19190 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19193 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19206 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 18910 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 18920 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18963 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18888 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18900 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 18934 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 19091 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19082 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 18989 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18956 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 18987 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18973 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18985 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 19001 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 19136 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18961 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19155 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18948 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19171 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19153 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18957 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19017 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 19030 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 19092 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18925 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18942 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 19135 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19137 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18970 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18988 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19126 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18926 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 19158 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 19053 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19062 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 18922 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19021 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19027 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19093 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 18976 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18994 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 18992 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 19055 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 19109 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 19121 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 19163 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 18889 {
            return Some(AndesiteWall {
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18966 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 19101 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 18917 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18927 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 19195 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 19050 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 19079 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 19196 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19202 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18921 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 18933 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19007 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19012 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 19178 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 18986 {
            return Some(AndesiteWall {
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19014 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19039 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 19071 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19142 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 19043 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 19081 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 19204 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18937 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19002 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 19076 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19097 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18886 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19122 {
            return Some(AndesiteWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 19180 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18996 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 18907 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 19147 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 18979 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19189 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 19123 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18941 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18944 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 19037 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 19134 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 18929 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18903 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 18939 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 18884 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 19069 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18897 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19128 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19032 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19172 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18887 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 19040 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18892 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18990 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19131 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19157 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19096 {
            return Some(AndesiteWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19181 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18893 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 19013 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 18998 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 19068 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 19203 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18885 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 19087 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18974 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 18923 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 19041 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18894 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 19154 {
            return Some(AndesiteWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18912 {
            return Some(AndesiteWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 19088 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 19176 {
            return Some(AndesiteWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19065 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 19177 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18997 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 19010 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 18991 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19138 {
            return Some(AndesiteWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19167 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 18918 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 19028 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 19188 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 19048 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 19000 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18919 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19199 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19029 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19073 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 18969 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 19057 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18999 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 19143 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19105 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19086 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19186 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 19051 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#up: false,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18954 {
            return Some(AndesiteWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 19005 {
            return Some(AndesiteWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 18914 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 19072 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19083 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18904 {
            return Some(AndesiteWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 19103 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19084 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 18982 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18895 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19011 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 19111 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19197 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18935 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 18953 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 18980 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#west: West::None,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19026 {
            return Some(AndesiteWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19036 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19080 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 19107 {
            return Some(AndesiteWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19170 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19175 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19152 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 19174 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18891 {
            return Some(AndesiteWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 19038 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 18906 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 19102 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19115 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 19118 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19075 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19150 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 19089 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18928 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18977 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 18995 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 19045 {
            return Some(AndesiteWall {
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19060 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 19056 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 19066 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 19077 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 19049 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 19124 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19139 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18965 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 19110 {
            return Some(AndesiteWall {
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19201 {
            return Some(AndesiteWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18902 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 18916 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 18960 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 19046 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18908 {
            return Some(AndesiteWall {
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18930 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 18964 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19020 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19173 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 19145 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19100 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19074 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19006 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19067 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 19106 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 18940 {
            return Some(AndesiteWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18968 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 18936 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 19058 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 19161 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18896 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 18938 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 19054 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 18981 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 18950 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 19112 {
            return Some(AndesiteWall {
                r#up: true,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19113 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18983 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 19022 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19185 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 19130 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19116 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 19149 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 19078 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19003 {
            return Some(AndesiteWall {
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19063 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19090 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19162 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 18909 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18947 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 19025 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 18890 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19033 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19104 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19144 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19146 {
            return Some(AndesiteWall {
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19156 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 19169 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19182 {
            return Some(AndesiteWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 19183 {
            return Some(AndesiteWall {
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19194 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 19198 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19200 {
            return Some(AndesiteWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19061 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 19004 {
            return Some(AndesiteWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18905 {
            return Some(AndesiteWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 19120 {
            return Some(AndesiteWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18984 {
            return Some(AndesiteWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 18962 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18899 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18972 {
            return Some(AndesiteWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19009 {
            return Some(AndesiteWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19016 {
            return Some(AndesiteWall {
                r#north: North::None,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19031 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19166 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18978 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18949 {
            return Some(AndesiteWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19019 {
            return Some(AndesiteWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19035 {
            return Some(AndesiteWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 19205 {
            return Some(AndesiteWall {
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 19023 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 19119 {
            return Some(AndesiteWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 19018 {
            return Some(AndesiteWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        return None;
    }
}

