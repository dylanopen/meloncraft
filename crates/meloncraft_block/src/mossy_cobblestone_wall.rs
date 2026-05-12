use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossyCobblestoneWall {
    pub up: bool,
    pub waterlogged: bool,
    pub r#east: East,
    pub r#south: South,
    pub r#west: West,
    pub r#north: North,
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

impl BlockState for MossyCobblestoneWall {
    fn to_id(self) -> i32 {
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::None { return 10401; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 10377; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == true { return 10251; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low { return 10189; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 10409; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 10427; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::None { return 10126; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low { return 10229; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::None { return 10113; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 10153; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Low { return 10111; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true { return 10181; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::None { return 10186; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 10352; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::Low { return 10279; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::None { return 10116; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Tall { return 10239; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true { return 10164; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 10190; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::None { return 10224; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low { return 10138; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 10290; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false { return 10187; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 10310; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::None { return 10212; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 10318; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 10142; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::None { return 10179; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 10382; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 10389; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall { return 10413; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall { return 10416; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 10424; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 10331; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 10383; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Low { return 10152; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::None { return 10266; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == false { return 10281; }
        if block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Tall { return 10325; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 10414; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 10235; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 10192; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::Low { return 10262; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 10418; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Tall { return 10294; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == true { return 10421; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 10147; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 10422; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None { return 10137; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::Tall { return 10204; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#waterlogged == false { return 10197; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 10415; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true { return 10395; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == true { return 10109; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 10210; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::None { return 10350; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 10124; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall { return 10275; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low { return 10378; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None { return 10119; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 10127; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::None { return 10243; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 10304; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == true { return 10335; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None { return 10128; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#east == East::Low { return 10249; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 10272; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 10412; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == false { return 10173; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::None { return 10185; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 10277; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 10267; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 10356; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 10419; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false { return 10208; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 10373; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None { return 10209; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::None { return 10236; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 10283; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 10169; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Tall { return 10199; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true { return 10218; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 10261; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false { return 10399; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 10364; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 10391; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 10405; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall { return 10247; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Tall { return 10176; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 10145; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == true { return 10188; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 10144; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Tall { return 10200; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::None { return 10220; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 10347; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false { return 10326; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::None { return 10110; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true { return 10123; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 10269; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 10160; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Tall { return 10323; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 10408; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true { return 10107; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low { return 10297; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::None { return 10338; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Tall { return 10132; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 10172; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 10327; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true { return 10167; }
        if block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 10311; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 10280; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Low { return 10368; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 10268; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 10299; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 10319; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Tall { return 10205; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low { return 10291; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == false { return 10341; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::Tall { return 10381; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None { return 10354; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 10365; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 10388; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 10390; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::None { return 10288; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::None { return 10333; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::None { return 10260; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 10345; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 10178; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::None { return 10155; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None { return 10151; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 10396; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None { return 10149; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 10206; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::None { return 10344; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 10366; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == false { return 10254; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true { return 10225; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 10300; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == false { return 10162; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 10353; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 10158; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Tall { return 10163; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 10301; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::None { return 10213; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#north == North::None { return 10214; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::None { return 10129; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None { return 10193; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == true { return 10346; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::None { return 10219; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None { return 10180; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 10157; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Low { return 10117; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 10358; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Low { return 10270; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::None { return 10253; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None { return 10256; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low { return 10242; }
        if block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Low { return 10143; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Low { return 10196; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == false { return 10184; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::None { return 10257; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Low { return 10246; }
        if block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false { return 10245; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::None { return 10161; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Low { return 10307; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == false { return 10122; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 10411; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#west == West::None { return 10407; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None { return 10371; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == false { return 10328; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Tall { return 10385; }
        if block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true { return 10106; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false { return 10379; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#west == West::None { return 10191; }
        if block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 10250; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == true { return 10348; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == false { return 10194; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::Tall { return 10340; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 10170; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall { return 10207; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::None { return 10231; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 10313; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall { return 10343; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::None { return 10362; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::Tall { return 10314; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::None { return 10397; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false { return 10139; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 10339; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false { return 10402; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::None { return 10259; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low { return 10255; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::None { return 10337; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 10369; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 10240; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None { return 10223; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 10182; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall { return 10303; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == true { return 10332; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == true { return 10285; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Low { return 10298; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 10361; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 10278; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 10125; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == true { return 10284; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None { return 10351; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 10312; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 10359; }
        if block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 10375; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low { return 10216; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::None { return 10165; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 10115; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 10393; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 10211; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None { return 10183; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low { return 10305; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true { return 10360; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 10287; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::None { return 10234; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#waterlogged == false { return 10120; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 10195; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true { return 10201; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Low { return 10264; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Tall { return 10131; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::Low { return 10271; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Tall { return 10130; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 10276; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 10286; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Low { return 10321; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false { return 10289; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Tall { return 10370; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::Tall { return 10398; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None { return 10403; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true { return 10406; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false { return 10112; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 10215; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 10380; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 10217; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 10426; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall { return 10317; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false { return 10135; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 10203; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 10238; }
        if block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 10226; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 10386; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::None { return 10104; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 10230; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true { return 10394; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::None { return 10330; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::None { return 10202; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == false { return 10306; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 10233; }
        if block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low { return 10258; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 10420; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 10274; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 10355; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::None { return 10146; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None { return 10322; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == false { return 10150; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::None { return 10329; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 10136; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#north == North::Tall { return 10177; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false { return 10114; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 10171; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == true { return 10140; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 10263; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false { return 10244; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::Low { return 10252; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 10198; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 10121; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low { return 10237; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 10387; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 10342; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::None { return 10349; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 10357; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::None { return 10302; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#up == false { return 10134; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Low { return 10156; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true { return 10320; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 10336; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#south == South::None { return 10392; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Tall { return 10133; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 10417; }
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 10425; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 10141; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 10166; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Tall { return 10316; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low { return 10108; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::Low { return 10309; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Tall { return 10232; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 10296; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall { return 10376; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 10222; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low { return 10248; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::None { return 10295; }
        if block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true { return 10148; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::Low { return 10227; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false { return 10159; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low { return 10228; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 10174; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::None { return 10221; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::None { return 10241; }
        if block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true { return 10292; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Low { return 10273; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == true { return 10105; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 10374; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true { return 10265; }
        if block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 10175; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 10282; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == true { return 10324; }
        if block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == false { return 10367; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 10363; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall { return 10315; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 10168; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None { return 10400; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 10372; }
        if block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Low { return 10154; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 10423; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::None { return 10293; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Tall { return 10118; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 10334; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 10308; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 10404; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 10410; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Low { return 10384; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10401 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 10377 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 10251 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 10189 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 10409 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 10427 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 10126 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 10229 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 10113 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 10153 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 10111 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 10181 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 10186 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 10352 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 10279 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 10116 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 10239 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 10164 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 10190 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10224 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 10138 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 10290 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 10187 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 10310 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 10212 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 10318 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 10142 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 10179 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 10382 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 10389 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 10413 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 10416 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 10424 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 10331 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10383 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 10152 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 10266 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 10281 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10325 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 10414 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 10235 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 10192 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 10262 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 10418 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 10294 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 10421 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 10147 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10422 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 10137 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 10204 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 10197 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10415 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 10395 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 10109 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 10210 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 10350 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 10124 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 10275 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 10378 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 10119 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 10127 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 10243 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 10304 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10335 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 10128 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 10249 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 10272 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 10412 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 10173 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 10185 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 10277 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 10267 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10356 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 10419 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 10208 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 10373 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10209 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 10236 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 10283 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 10169 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10199 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 10218 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10261 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10399 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 10364 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 10391 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10405 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 10247 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 10176 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 10145 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 10188 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 10144 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 10200 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 10220 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 10347 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 10326 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 10110 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 10123 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10269 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10160 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10323 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 10408 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10107 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 10297 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 10338 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 10132 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
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
        if state_id == 10327 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 10167 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 10311 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 10280 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 10368 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 10268 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 10299 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 10319 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 10205 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 10291 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 10341 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 10381 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 10354 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 10365 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 10388 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 10390 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 10288 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 10333 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 10260 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 10345 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 10178 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 10155 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 10151 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 10396 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 10149 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 10206 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 10344 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 10366 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 10254 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 10225 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 10300 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 10162 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 10353 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10158 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10163 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10301 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10213 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 10214 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 10129 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 10193 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 10346 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 10219 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 10180 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 10157 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 10117 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 10358 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 10270 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 10253 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 10256 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 10242 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 10143 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 10196 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 10184 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 10257 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 10246 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 10245 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 10161 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 10307 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 10122 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 10411 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 10407 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 10371 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 10328 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 10385 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10106 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10379 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 10191 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 10250 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 10348 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 10194 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 10340 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 10170 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 10207 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 10231 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 10313 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 10343 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10362 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 10314 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 10397 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 10139 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10339 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 10402 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10259 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 10255 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 10337 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 10369 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10240 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10223 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 10182 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 10303 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 10332 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 10285 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 10298 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 10361 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10278 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 10125 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 10284 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 10351 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 10312 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 10359 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10375 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10216 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 10165 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 10115 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10393 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 10211 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10183 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 10305 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 10360 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 10287 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 10234 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 10120 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10195 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10201 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 10264 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 10131 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 10271 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 10130 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 10276 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10286 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10321 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 10289 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10370 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 10398 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 10403 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 10406 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 10112 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 10215 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 10380 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 10217 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 10426 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 10317 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 10135 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 10203 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10238 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 10226 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 10386 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 10104 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 10230 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 10394 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 10330 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 10202 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 10306 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 10233 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 10258 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 10420 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 10274 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 10355 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 10146 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 10322 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 10150 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 10329 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 10136 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10177 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 10114 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 10171 {
            return Some(MossyCobblestoneWall {
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 10140 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10263 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 10244 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 10252 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 10198 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 10121 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10237 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 10387 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 10342 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 10349 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 10357 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 10302 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 10134 {
            return Some(MossyCobblestoneWall {
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 10156 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 10320 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 10336 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 10392 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 10133 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 10417 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 10425 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 10141 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 10166 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 10316 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 10108 {
            return Some(MossyCobblestoneWall {
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 10309 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 10232 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 10296 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 10376 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 10222 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 10248 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 10295 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 10148 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10227 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 10159 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 10228 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 10174 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 10221 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 10241 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 10292 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10273 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 10105 {
            return Some(MossyCobblestoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 10374 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 10265 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 10175 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 10282 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 10324 {
            return Some(MossyCobblestoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 10367 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 10363 {
            return Some(MossyCobblestoneWall {
                r#up: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 10315 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 10168 {
            return Some(MossyCobblestoneWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 10400 {
            return Some(MossyCobblestoneWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
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
        if state_id == 10154 {
            return Some(MossyCobblestoneWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 10423 {
            return Some(MossyCobblestoneWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 10293 {
            return Some(MossyCobblestoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 10118 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 10334 {
            return Some(MossyCobblestoneWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 10308 {
            return Some(MossyCobblestoneWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 10404 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 10410 {
            return Some(MossyCobblestoneWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 10384 {
            return Some(MossyCobblestoneWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        return None;
    }
}

