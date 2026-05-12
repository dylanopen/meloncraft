use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuffBrickWall {
    pub r#north: North,
    pub up: bool,
    pub r#west: West,
    pub waterlogged: bool,
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

impl BlockState for TuffBrickWall {
    fn to_id(self) -> i32 {
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 24230; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::None { return 24217; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low { return 24215; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true { return 24364; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true { return 24318; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#east == East::None { return 24249; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false { return 24481; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low { return 24323; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None { return 24280; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == false { return 24469; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == false { return 24170; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Low { return 24248; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 24443; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 24477; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::None { return 24173; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 24240; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#south == South::None { return 24351; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#waterlogged == true { return 24227; }
        if block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Low { return 24305; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 24402; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == false { return 24407; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true { return 24412; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low { return 24427; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 24297; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 24460; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::None { return 24186; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::None { return 24251; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 24478; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false { return 24195; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::None { return 24459; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None { return 24238; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 24203; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low { return 24416; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::Low { return 24424; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Low { return 24197; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::None { return 24295; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Tall { return 24379; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 24442; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false { return 24194; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::None { return 24224; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#south == South::Low { return 24247; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 24193; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 24405; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 24414; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 24420; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 24228; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::None { return 24272; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None { return 24382; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Low { return 24433; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low { return 24392; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Low { return 24180; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#west == West::Tall { return 24471; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 24429; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::None { return 24178; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Low { return 24476; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None { return 24204; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None { return 24352; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None { return 24226; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == false { return 24301; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low { return 24311; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == false { return 24421; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true { return 24167; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 24430; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::None { return 24189; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 24438; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == true { return 24256; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 24361; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 24391; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 24404; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::Low { return 24320; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 24255; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == false { return 24307; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::None { return 24376; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == false { return 24277; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true { return 24184; }
        if block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall { return 24385; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 24398; }
        if block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false { return 24312; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 24330; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Low { return 24176; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 24450; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::None { return 24422; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::None { return 24254; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Tall { return 24415; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 24319; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 24258; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#south == South::Tall { return 24298; }
        if block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false { return 24454; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 24483; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == false { return 24214; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 24375; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::None { return 24211; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#west == West::Tall { return 24291; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low { return 24188; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == false { return 24200; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None { return 24253; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == false { return 24396; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Low { return 24208; }
        if block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall { return 24384; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 24233; }
        if block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 24225; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None { return 24168; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall { return 24300; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 24399; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::None { return 24241; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Low { return 24357; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 24175; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 24324; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::Low { return 24380; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::None { return 24286; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::None { return 24223; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == false { return 24339; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::Tall { return 24387; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Low { return 24432; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::None { return 24192; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#up == true { return 24294; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Low { return 24425; }
        if block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 24437; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None { return 24190; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::Low { return 24388; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 24445; }
        if block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == true { return 24461; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 24470; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::None { return 24275; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::None { return 24274; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 24378; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 24269; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Tall { return 24354; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == true { return 24328; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 24216; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 24340; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 24365; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Low { return 24389; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 24327; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 24244; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == false { return 24315; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 24374; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Low { return 24207; }
        if block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::None { return 24394; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == false { return 24397; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == false { return 24435; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::Tall { return 24462; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::None { return 24290; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low { return 24468; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#north == North::Low { return 24309; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 24446; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall { return 24363; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall { return 24482; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false { return 24467; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 24292; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 24358; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#up == false { return 24276; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::None { return 24341; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#up == true { return 24345; }
        if block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true { return 24418; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true { return 24406; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None { return 24263; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::Tall { return 24453; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None { return 24169; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Tall { return 24162; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None { return 24198; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 24273; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 24299; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::None { return 24349; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Tall { return 24369; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 24400; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 24372; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#up == true { return 24209; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 24266; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::None { return 24390; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == true { return 24408; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::None { return 24160; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 24436; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 24444; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 24458; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::None { return 24451; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 24243; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 24356; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false { return 24419; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::None { return 24236; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 24252; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 24221; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 24455; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Low { return 24308; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true { return 24172; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::Low { return 24179; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Tall { return 24270; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None { return 24181; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 24347; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true { return 24234; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 24260; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Low { return 24287; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true { return 24316; }
        if block_state.r#up == false && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 24434; }
        if block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 24335; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Tall { return 24267; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 24447; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low { return 24359; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false { return 24373; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == false { return 24296; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::Low { return 24206; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Low { return 24317; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None { return 24456; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true { return 24232; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None { return 24331; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 24343; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Low { return 24239; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::None { return 24457; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#west == West::Low { return 24212; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Tall { return 24231; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 24479; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Tall { return 24428; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Low { return 24205; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Low { return 24285; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::Tall { return 24306; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall { return 24336; }
        if block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false { return 24314; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 24480; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 24472; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low { return 24313; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true { return 24475; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#waterlogged == true { return 24346; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#north == North::None { return 24187; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::None { return 24191; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 24413; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None { return 24171; }
        if block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 24218; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == false { return 24302; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#north == North::Low { return 24304; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low { return 24371; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low { return 24182; }
        if block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::None { return 24242; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == true { return 24452; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::None { return 24174; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 24383; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#north == North::Low { return 24202; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 24322; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 24326; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#west == West::Tall { return 24333; }
        if block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::None { return 24250; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None { return 24409; }
        if block_state.r#west == West::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::None { return 24199; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::None && block_state.r#south == South::Tall { return 24401; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 24321; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 24342; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 24386; }
        if block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::Tall { return 24338; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true { return 24441; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::Low { return 24268; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 24368; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 24245; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::None { return 24166; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Low { return 24362; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 24282; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::Low { return 24466; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false { return 24334; }
        if block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 24410; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 24474; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == false { return 24395; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 24355; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 24353; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true { return 24261; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#south == South::Tall { return 24265; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Low { return 24288; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#north == North::None { return 24165; }
        if block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall { return 24377; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == true { return 24235; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Low { return 24201; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 24210; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 24229; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::None { return 24259; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall { return 24417; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == true { return 24439; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall { return 24440; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 24464; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low { return 24350; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == true { return 24403; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::None { return 24463; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::None { return 24271; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Low { return 24183; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::None { return 24278; }
        if block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 24325; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::None { return 24344; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 24332; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::None { return 24381; }
        if block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false { return 24411; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == false { return 24264; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 24303; }
        if block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#up == false { return 24360; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#east == East::Low { return 24337; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#up == true { return 24367; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Low { return 24426; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Tall { return 24257; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Tall { return 24449; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true { return 24281; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#west == West::Tall { return 24366; }
        if block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false { return 24164; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Tall { return 24262; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 24329; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None { return 24348; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None { return 24213; }
        if block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Low { return 24196; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::None { return 24185; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 24473; }
        if block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low { return 24310; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Low { return 24283; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true { return 24161; }
        if block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::None { return 24393; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::Tall { return 24431; }
        if block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#east == East::None { return 24237; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false { return 24465; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::None { return 24293; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall { return 24246; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#up == true { return 24177; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#east == East::None && block_state.r#waterlogged == true { return 24222; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 24284; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#up == false && block_state.r#south == South::Low { return 24289; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#west == West::None { return 24163; }
        if block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::None { return 24220; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Tall { return 24370; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall { return 24423; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 24219; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::None { return 24448; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false { return 24279; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24230 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24217 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 24215 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 24364 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 24318 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 24249 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 24481 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 24323 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 24280 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 24469 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 24170 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 24248 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 24443 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 24477 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 24173 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 24240 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24351 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 24227 {
            return Some(TuffBrickWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24305 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 24402 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 24407 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 24412 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 24427 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 24297 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 24460 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 24186 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 24251 {
            return Some(TuffBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 24478 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 24195 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 24459 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 24238 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 24203 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 24416 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 24424 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 24197 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 24295 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 24379 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 24442 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24194 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 24224 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 24247 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 24193 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24405 {
            return Some(TuffBrickWall {
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24414 {
            return Some(TuffBrickWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 24420 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 24228 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 24272 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 24382 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 24433 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 24392 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 24180 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 24471 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 24429 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24178 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 24476 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 24204 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 24352 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 24226 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 24301 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 24311 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 24421 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 24167 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 24430 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24189 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 24438 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 24256 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 24361 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 24391 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: true,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 24404 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 24320 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 24255 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 24307 {
            return Some(TuffBrickWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 24376 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 24277 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 24184 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 24385 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 24398 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 24312 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 24330 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 24176 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 24450 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 24422 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 24254 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 24415 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 24319 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 24258 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 24298 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 24454 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 24483 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24214 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 24375 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 24211 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 24291 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 24188 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 24200 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 24253 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 24396 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 24208 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 24384 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 24233 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 24225 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 24168 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 24300 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 24399 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 24241 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 24357 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 24175 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 24324 {
            return Some(TuffBrickWall {
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 24380 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 24286 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 24223 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 24339 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 24387 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 24432 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 24192 {
            return Some(TuffBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 24294 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 24425 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 24437 {
            return Some(TuffBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 24190 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 24388 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 24445 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 24461 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 24470 {
            return Some(TuffBrickWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24275 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 24274 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 24378 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 24269 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24354 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24328 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 24216 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 24340 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 24365 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 24389 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 24327 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 24244 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24315 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 24374 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 24207 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 24394 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 24397 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 24435 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 24462 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 24290 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 24468 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24309 {
            return Some(TuffBrickWall {
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 24446 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 24363 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 24482 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 24467 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 24292 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 24358 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 24276 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 24341 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 24345 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 24418 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 24406 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24263 {
            return Some(TuffBrickWall {
                r#up: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 24453 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 24169 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 24162 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24198 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 24273 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 24299 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 24349 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 24369 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 24400 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 24372 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24209 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 24266 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 24390 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 24408 {
            return Some(TuffBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24160 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 24436 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 24444 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 24458 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 24451 {
            return Some(TuffBrickWall {
                r#up: true,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 24243 {
            return Some(TuffBrickWall {
                r#up: false,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 24356 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 24419 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 24236 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 24252 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 24221 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 24455 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 24308 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 24172 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 24179 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 24270 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24181 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 24347 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 24234 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 24260 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 24287 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 24316 {
            return Some(TuffBrickWall {
                r#up: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 24434 {
            return Some(TuffBrickWall {
                r#up: false,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 24335 {
            return Some(TuffBrickWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 24267 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 24447 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 24359 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 24373 {
            return Some(TuffBrickWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24296 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 24206 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 24317 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 24456 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 24232 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24331 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 24343 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 24239 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 24457 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 24212 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 24231 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24479 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 24428 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 24205 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 24285 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 24306 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 24336 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 24314 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 24480 {
            return Some(TuffBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 24472 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 24313 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 24475 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 24346 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24187 {
            return Some(TuffBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 24191 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 24413 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 24171 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 24218 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 24302 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 24304 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 24371 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 24182 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 24242 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 24452 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 24174 {
            return Some(TuffBrickWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 24383 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 24202 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 24322 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 24326 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 24333 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 24250 {
            return Some(TuffBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 24409 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 24199 {
            return Some(TuffBrickWall {
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 24401 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: true,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 24321 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 24342 {
            return Some(TuffBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 24386 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24338 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 24441 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 24268 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 24368 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 24245 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 24166 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 24362 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 24282 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24466 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 24334 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 24410 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 24474 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24395 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 24355 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 24353 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 24261 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 24265 {
            return Some(TuffBrickWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 24288 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 24165 {
            return Some(TuffBrickWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 24377 {
            return Some(TuffBrickWall {
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 24235 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 24201 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 24210 {
            return Some(TuffBrickWall {
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24229 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 24259 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 24417 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 24439 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 24440 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 24464 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 24350 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 24403 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 24463 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 24271 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 24183 {
            return Some(TuffBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 24278 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 24325 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 24344 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 24332 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 24381 {
            return Some(TuffBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 24411 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 24264 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 24303 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 24360 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 24337 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 24367 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 24426 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 24257 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 24449 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 24281 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24366 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 24164 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 24262 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 24329 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 24348 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 24213 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 24196 {
            return Some(TuffBrickWall {
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 24185 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 24473 {
            return Some(TuffBrickWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 24310 {
            return Some(TuffBrickWall {
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 24283 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 24161 {
            return Some(TuffBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 24393 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 24431 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 24237 {
            return Some(TuffBrickWall {
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 24465 {
            return Some(TuffBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 24293 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 24246 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 24177 {
            return Some(TuffBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 24222 {
            return Some(TuffBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 24284 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 24289 {
            return Some(TuffBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 24163 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 24220 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 24370 {
            return Some(TuffBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 24423 {
            return Some(TuffBrickWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 24219 {
            return Some(TuffBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 24448 {
            return Some(TuffBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 24279 {
            return Some(TuffBrickWall {
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

