use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossyStoneBrickWall {
    pub up: bool,
    pub r#west: West,
    pub r#north: North,
    pub r#east: East,
    pub waterlogged: bool,
    pub r#south: South,
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

impl BlockState for MossyStoneBrickWall {
    fn to_id(&self) -> i32 {
        if self.r#north == North::None && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall { return 17482; }
        if self.r#north == North::Tall && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall { return 17576; }
        if self.r#up == false && self.r#west == West::None && self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::Low && self.r#east == East::None { return 17285; }
        if self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == true { return 17516; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::None && self.r#up == true { return 17555; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::Low && self.r#west == West::Low && self.r#up == false { return 17499; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == false { return 17572; }
        if self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false { return 17271; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::Tall { return 17565; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#up == false && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::None { return 17309; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#up == true && self.r#east == East::None && self.r#west == West::None { return 17312; }
        if self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::None && self.r#east == East::Low { return 17378; }
        if self.r#up == false && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Low { return 17356; }
        if self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None && self.r#north == North::None { return 17375; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Low && self.r#up == false { return 17498; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None { return 17507; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#up == true { return 17401; }
        if self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::Tall { return 17554; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Tall { return 17560; }
        if self.r#up == false && self.r#east == East::Tall && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::None { return 17562; }
        if self.r#east == East::Tall && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall { return 17566; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Tall { return 17585; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == false { return 17334; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#west == West::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == false { return 17514; }
        if self.r#west == West::Tall && self.r#waterlogged == true && self.r#up == false && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::None { return 17500; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#up == true { return 17268; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#up == false { return 17392; }
        if self.r#west == West::Tall && self.r#waterlogged == false && self.r#up == false && self.r#north == North::Low && self.r#south == South::Tall && self.r#east == East::Low { return 17443; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::Tall && self.r#west == West::Tall { return 17479; }
        if self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::Tall { return 17364; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Low { return 17537; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == true && self.r#south == South::None && self.r#west == West::Low && self.r#north == North::Tall { return 17556; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == false { return 17574; }
        if self.r#north == North::None && self.r#west == West::Low && self.r#waterlogged == true && self.r#up == true && self.r#east == East::None && self.r#south == South::Low { return 17277; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#up == true && self.r#east == East::Low { return 17444; }
        if self.r#east == East::None && self.r#up == false && self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Tall { return 17296; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::None && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true { return 17278; }
        if self.r#waterlogged == true && self.r#up == true && self.r#east == East::Low && self.r#south == South::None && self.r#west == West::Low && self.r#north == North::Tall { return 17445; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Tall { return 17502; }
        if self.r#waterlogged == false && self.r#up == false && self.r#west == West::None && self.r#south == South::None && self.r#east == East::None && self.r#north == North::Tall { return 17345; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == true && self.r#south == South::Low && self.r#east == East::Low { return 17457; }
        if self.r#up == true && self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None { return 17531; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false { return 17298; }
        if self.r#up == false && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::None && self.r#north == North::None && self.r#west == West::Tall { return 17275; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::None && self.r#up == false { return 17307; }
        if self.r#up == true && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::None && self.r#west == West::None && self.r#east == East::None { return 17288; }
        if self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::None { return 17311; }
        if self.r#south == South::None && self.r#up == true && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low { return 17305; }
        if self.r#north == North::Low && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None { return 17331; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::None && self.r#up == true { return 17374; }
        if self.r#up == false && self.r#south == South::None && self.r#north == North::Low && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low { return 17310; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Tall { return 17398; }
        if self.r#up == true && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Low { return 17456; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::None && self.r#north == North::None && self.r#up == false && self.r#west == West::Low { return 17490; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::None { return 17300; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None { return 17513; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#up == true { return 17373; }
        if self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == false { return 17536; }
        if self.r#east == East::None && self.r#north == North::None && self.r#south == South::Tall && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == false { return 17299; }
        if self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::None && self.r#south == South::None && self.r#up == true { return 17269; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == false && self.r#south == South::Tall { return 17515; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::Low && self.r#up == false { return 17474; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None && self.r#north == North::Tall && self.r#up == false { return 17561; }
        if self.r#north == North::None && self.r#east == East::None && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low { return 17265; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::Low { return 17419; }
        if self.r#east == East::Low && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false && self.r#south == South::None { return 17454; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None { return 17509; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Tall { return 17539; }
        if self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::Tall && self.r#west == West::Low { return 17583; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == false && self.r#north == North::Low { return 17550; }
        if self.r#up == false && self.r#west == West::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Low { return 17462; }
        if self.r#west == West::Tall && self.r#up == false && self.r#south == South::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::None { return 17380; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::None && self.r#west == West::Low && self.r#up == true && self.r#south == South::Low { return 17280; }
        if self.r#north == North::Tall && self.r#up == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false { return 17467; }
        if self.r#west == West::None && self.r#up == true && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == false { return 17387; }
        if self.r#up == true && self.r#south == South::Tall && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == true { return 17504; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#up == false && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::Low { return 17318; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#up == true { return 17399; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#up == false && self.r#east == East::Low { return 17418; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true && self.r#east == East::None && self.r#waterlogged == true { return 17360; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == false { return 17478; }
        if self.r#up == false && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::None && self.r#west == West::Low && self.r#east == East::Tall { return 17523; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Tall && self.r#up == true { return 17292; }
        if self.r#north == North::Tall && self.r#west == West::Tall && self.r#up == false && self.r#south == South::None && self.r#east == East::Tall && self.r#waterlogged == false { return 17563; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Tall && self.r#east == East::Tall { return 17545; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false { return 17501; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == false { return 17575; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == true && self.r#west == West::None { return 17579; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Low { return 17420; }
        if self.r#waterlogged == false && self.r#up == true && self.r#east == East::Low && self.r#west == West::Low && self.r#south == South::None && self.r#north == North::Tall { return 17448; }
        if self.r#east == East::Low && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None { return 17405; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Tall && self.r#east == East::None { return 17338; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == true && self.r#east == East::Tall && self.r#west == West::Tall && self.r#south == South::Low { return 17569; }
        if self.r#up == true && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Low { return 17281; }
        if self.r#south == South::Tall && self.r#west == West::Low && self.r#east == East::Low && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true { return 17475; }
        if self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::None && self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall { return 17349; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Tall { return 17458; }
        if self.r#west == West::Low && self.r#east == East::Tall && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::Low { return 17538; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::None && self.r#south == South::Low && self.r#east == East::None && self.r#up == false { return 17321; }
        if self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Low { return 17430; }
        if self.r#south == South::None && self.r#north == North::Low && self.r#up == true && self.r#east == East::Low && self.r#west == West::Tall && self.r#waterlogged == false { return 17413; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Tall { return 17570; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::None && self.r#up == false && self.r#west == West::None { return 17294; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#up == false && self.r#west == West::None { return 17426; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Low && self.r#up == true && self.r#west == West::None { return 17447; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::None && self.r#up == true && self.r#north == North::Low { return 17304; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::None && self.r#up == true && self.r#west == West::Low { return 17508; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#south == South::Tall { return 17581; }
        if self.r#north == North::None && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::None { return 17495; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Tall && self.r#up == false && self.r#south == South::Tall && self.r#east == East::Tall { return 17582; }
        if self.r#south == South::Tall && self.r#up == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == true { return 17542; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#up == true && self.r#south == South::Low && self.r#east == East::Tall && self.r#waterlogged == false { return 17497; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None { return 17477; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::Low && self.r#up == true { return 17314; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false { return 17293; }
        if self.r#east == East::None && self.r#up == false && self.r#south == South::None && self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::None { return 17272; }
        if self.r#up == true && self.r#south == South::Low && self.r#east == East::None && self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::Tall { return 17351; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall { return 17586; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == true { return 17557; }
        if self.r#east == East::Tall && self.r#west == West::None && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None { return 17489; }
        if self.r#west == West::Low && self.r#north == North::None && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::None { return 17379; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false && self.r#north == North::Tall { return 17358; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false && self.r#south == South::Tall && self.r#west == West::Tall && self.r#waterlogged == false { return 17587; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true { return 17344; }
        if self.r#east == East::Tall && self.r#up == false && self.r#west == West::None && self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::None { return 17486; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true { return 17402; }
        if self.r#up == true && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Low { return 17389; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#up == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::Low { return 17359; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None { return 17289; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#up == false && self.r#south == South::None && self.r#north == North::None && self.r#west == West::Low { return 17487; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false && self.r#west == West::None && self.r#east == East::None && self.r#south == South::Low { return 17357; }
        if self.r#west == West::None && self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None { return 17381; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::None && self.r#south == South::None { return 17450; }
        if self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low && self.r#east == East::Low { return 17394; }
        if self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#up == true { return 17519; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false { return 17322; }
        if self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::None && self.r#north == North::Low { return 17303; }
        if self.r#up == false && self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::Tall { return 17525; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::None && self.r#north == North::Low && self.r#up == true { return 17520; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == true && self.r#north == North::None { return 17505; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#up == true { return 17317; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::Low && self.r#up == false && self.r#west == West::Tall { return 17332; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::None { return 17327; }
        if self.r#west == West::Low && self.r#south == South::None && self.r#north == North::None && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false { return 17484; }
        if self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#north == North::Tall { return 17363; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 17473; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low { return 17532; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low { return 17429; }
        if self.r#up == true && self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall { return 17400; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Tall && self.r#north == North::None && self.r#up == false { return 17491; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::Low && self.r#up == true { return 17553; }
        if self.r#north == North::Tall && self.r#up == true && self.r#south == South::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::None { return 17348; }
        if self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Low { return 17320; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None && self.r#south == South::None && self.r#north == North::Tall && self.r#up == true { return 17339; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false && self.r#up == false { return 17382; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true && self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::None { return 17432; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#west == West::None && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false { return 17333; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#east == East::None { return 17369; }
        if self.r#north == North::None && self.r#west == West::None && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#up == true { return 17291; }
        if self.r#west == West::Tall && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::None { return 17416; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#up == true && self.r#south == South::Tall && self.r#west == West::Low { return 17328; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Tall && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true { return 17366; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::None && self.r#up == true && self.r#west == West::None { return 17336; }
        if self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::None && self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == true { return 17446; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall { return 17361; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#south == South::None && self.r#up == false && self.r#north == North::Tall { return 17342; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == true && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Low { return 17567; }
        if self.r#east == East::Low && self.r#up == true && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::None { return 17372; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::None { return 17511; }
        if self.r#south == South::Tall && self.r#up == false && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 17368; }
        if self.r#west == West::Low && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::None { return 17385; }
        if self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall && self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::None { return 17347; }
        if self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 17461; }
        if self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::None && self.r#south == South::Low && self.r#up == true && self.r#north == North::None { return 17279; }
        if self.r#west == West::Tall && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::Tall { return 17584; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::None && self.r#up == true && self.r#east == East::None { return 17341; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Low && self.r#up == false { return 17452; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Low && self.r#up == false && self.r#west == West::Tall { return 17464; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#up == true && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Tall { return 17485; }
        if self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == true { return 17492; }
        if self.r#west == West::None && self.r#north == North::None && self.r#waterlogged == false && self.r#up == false && self.r#east == East::None && self.r#south == South::None { return 17273; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low && self.r#up == true && self.r#east == East::Low { return 17376; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::None { return 17481; }
        if self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::None { return 17343; }
        if self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::None { return 17283; }
        if self.r#south == South::Low && self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::None && self.r#up == true { return 17384; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#up == false && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::None { return 17319; }
        if self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Tall && self.r#up == false { return 17297; }
        if self.r#waterlogged == true && self.r#up == true && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::None && self.r#west == West::Low { return 17313; }
        if self.r#up == false && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall { return 17370; }
        if self.r#up == false && self.r#south == South::Low && self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 17428; }
        if self.r#north == North::None && self.r#up == true && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::None { return 17483; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Tall { return 17551; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 17350; }
        if self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Low && self.r#up == false { return 17431; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#up == false && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Tall { return 17441; }
        if self.r#west == West::Tall && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::Low && self.r#north == North::Low { return 17425; }
        if self.r#south == South::None && self.r#north == North::Low && self.r#west == West::Low && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == false { return 17412; }
        if self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::Low { return 17459; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == true { return 17568; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::None && self.r#waterlogged == false && self.r#up == false && self.r#north == North::None { return 17393; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::Low && self.r#up == true { return 17326; }
        if self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false { return 17352; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Low { return 17409; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == true { return 17552; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::None && self.r#up == true && self.r#west == West::None && self.r#north == North::Low { return 17408; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#up == false { return 17546; }
        if self.r#up == true && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::Tall { return 17518; }
        if self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == false { return 17510; }
        if self.r#up == false && self.r#south == South::None && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low { return 17274; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Low && self.r#up == true { return 17424; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true { return 17422; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Low && self.r#up == true && self.r#east == East::Low { return 17423; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#south == South::Tall { return 17436; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Low { return 17442; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == false && self.r#north == North::None { return 17503; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == true && self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::Tall { return 17577; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::Low { return 17403; }
        if self.r#west == West::Tall && self.r#east == East::Tall && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::None { return 17527; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#up == true && self.r#north == North::None { return 17276; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false { return 17455; }
        if self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Low && self.r#up == false { return 17417; }
        if self.r#east == East::Low && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Low { return 17435; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::Low && self.r#up == true && self.r#south == South::Low { return 17460; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Tall && self.r#east == East::Low && self.r#up == false { return 17438; }
        if self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::None { return 17404; }
        if self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::Tall && self.r#south == South::Tall && self.r#north == North::None { return 17512; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#west == West::None && self.r#waterlogged == true { return 17522; }
        if self.r#waterlogged == true && self.r#up == false && self.r#east == East::None && self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::None { return 17308; }
        if self.r#east == East::Low && self.r#west == West::None && self.r#up == true && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == false { return 17411; }
        if self.r#south == South::None && self.r#up == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Low { return 17526; }
        if self.r#west == West::None && self.r#east == East::None && self.r#up == false && self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::None { return 17270; }
        if self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Low && self.r#west == West::Tall && self.r#north == North::None { return 17383; }
        if self.r#south == South::None && self.r#up == false && self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Low { return 17415; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == false { return 17353; }
        if self.r#south == South::None && self.r#up == true && self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Tall { return 17521; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Tall && self.r#up == true && self.r#north == North::None && self.r#east == East::None { return 17290; }
        if self.r#north == North::Low && self.r#up == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#waterlogged == false { return 17544; }
        if self.r#south == South::Low && self.r#up == false && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == true { return 17284; }
        if self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Tall && self.r#west == West::Low { return 17571; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall { return 17371; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low { return 17465; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#up == true && self.r#south == South::Low && self.r#east == East::Low && self.r#west == West::Low { return 17421; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Tall && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true { return 17476; }
        if self.r#up == true && self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::None { return 17325; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low && self.r#up == false && self.r#south == South::Tall { return 17439; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Low { return 17541; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::Tall { return 17578; }
        if self.r#east == East::Low && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::None { return 17449; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::None && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false { return 17316; }
        if self.r#west == West::None && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == false && self.r#north == North::Tall { return 17354; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::None && self.r#up == true && self.r#west == West::Tall && self.r#south == South::Low { return 17494; }
        if self.r#south == South::None && self.r#west == West::None && self.r#waterlogged == true && self.r#up == false && self.r#north == North::Tall && self.r#east == East::Tall { return 17558; }
        if self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Low { return 17543; }
        if self.r#up == false && self.r#south == South::None && self.r#west == West::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#north == North::Tall { return 17451; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Low { return 17470; }
        if self.r#south == South::None && self.r#west == West::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#up == true && self.r#north == North::Low { return 17517; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#up == true && self.r#west == West::None && self.r#north == North::None && self.r#south == South::None { return 17267; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low { return 17529; }
        if self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Low { return 17286; }
        if self.r#up == true && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low { return 17433; }
        if self.r#north == North::Low && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None { return 17335; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None && self.r#up == true { return 17480; }
        if self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == true && self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::Low { return 17540; }
        if self.r#waterlogged == true && self.r#up == true && self.r#south == South::Tall && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Tall { return 17469; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#up == true { return 17580; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false { return 17346; }
        if self.r#up == true && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::None { return 17377; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#up == false && self.r#west == West::Low { return 17355; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 17434; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#up == false && self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::Tall { return 17463; }
        if self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true { return 17547; }
        if self.r#north == North::None && self.r#east == East::None && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 17282; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Tall { return 17365; }
        if self.r#south == South::Low && self.r#east == East::Low && self.r#west == West::None && self.r#up == false && self.r#north == North::None && self.r#waterlogged == true { return 17390; }
        if self.r#north == North::None && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low { return 17287; }
        if self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#north == North::None { return 17388; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true && self.r#south == South::Low { return 17530; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == true && self.r#west == West::None { return 17471; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Tall && self.r#west == West::None && self.r#up == false { return 17330; }
        if self.r#north == North::Tall && self.r#up == false && self.r#east == East::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Low { return 17573; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low { return 17406; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true { return 17533; }
        if self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true { return 17506; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#west == West::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#up == false { return 17440; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::None && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Low { return 17302; }
        if self.r#north == North::Tall && self.r#up == true && self.r#east == East::Tall && self.r#south == South::Low && self.r#west == West::None && self.r#waterlogged == true { return 17564; }
        if self.r#south == South::None && self.r#north == North::Low && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 17410; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 17466; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#up == false && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::None { return 17414; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#up == false && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Tall { return 17453; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 17524; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::None && self.r#south == South::None && self.r#up == true && self.r#west == West::None { return 17264; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Low && self.r#up == true && self.r#south == South::Tall { return 17324; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::None && self.r#up == false { return 17559; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::None && self.r#up == false { return 17295; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::None { return 17315; }
        if self.r#up == true && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 17386; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false { return 17437; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == false && self.r#east == East::Tall && self.r#west == West::None && self.r#north == North::Low { return 17549; }
        if self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == false { return 17427; }
        if self.r#east == East::Tall && self.r#west == West::Tall && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::None { return 17488; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == false && self.r#south == South::Low && self.r#west == West::None { return 17534; }
        if self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low { return 17340; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::Low && self.r#up == true { return 17493; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::None && self.r#up == true && self.r#south == South::Tall && self.r#east == East::Low { return 17396; }
        if self.r#up == false && self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None { return 17407; }
        if self.r#south == South::None && self.r#north == North::None && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 17266; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#up == false && self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Tall { return 17395; }
        if self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::None && self.r#south == South::Low { return 17528; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Low && self.r#south == South::Low { return 17391; }
        if self.r#up == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Low { return 17535; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true { return 17472; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == false { return 17367; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true && self.r#south == South::Tall && self.r#east == East::None { return 17362; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#east == East::None { return 17301; }
        if self.r#north == North::Low && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::None { return 17306; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::Low && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true { return 17548; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::None && self.r#up == true { return 17337; }
        if self.r#north == North::None && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Low && self.r#east == East::Low { return 17397; }
        if self.r#east == East::Low && self.r#west == West::None && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::Tall { return 17468; }
        if self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false { return 17323; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low { return 17329; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::None && self.r#up == true && self.r#west == West::Low { return 17496; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 17482 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17576 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 17285 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 17516 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 17555 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17499 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17572 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 17271 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17565 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 17309 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 17312 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 17378 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 17356 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17375 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
            });
        }
        if state_id == 17498 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 17507 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17401 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17554 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17560 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 17562 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17566 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17585 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17334 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17514 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 17500 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17268 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17392 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 17443 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17479 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17364 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17537 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 17556 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17574 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17277 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 17444 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 17296 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17278 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17445 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17502 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 17345 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17457 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17531 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 17298 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17275 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 17307 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 17288 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 17311 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 17305 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17331 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 17374 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17310 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17398 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17456 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 17490 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 17300 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 17513 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 17373 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17536 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 17299 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17269 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17515 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17474 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 17561 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 17265 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17419 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 17454 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 17509 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17539 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17583 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 17550 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 17462 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17380 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17280 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 17467 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17387 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17504 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17318 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 17399 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17418 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 17360 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17478 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17523 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17292 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17563 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17545 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17501 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17575 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 17579 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17420 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17448 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17405 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 17338 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17569 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17281 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 17475 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17349 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17458 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 17538 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 17321 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 17430 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17413 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17570 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17294 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 17426 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 17447 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17304 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 17508 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 17581 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17495 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 17582 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17542 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17497 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17477 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 17314 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 17293 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17272 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17351 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17586 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17557 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 17489 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 17379 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 17358 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17587 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17344 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17486 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 17402 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17389 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 17359 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17289 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 17487 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 17357 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 17381 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 17450 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 17394 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17519 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17322 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 17303 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17525 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17520 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 17505 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 17317 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 17332 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17327 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17484 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17363 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 17473 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17532 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 17429 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 17400 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17491 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 17553 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 17348 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17320 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17339 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17382 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 17432 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 17333 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17369 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 17291 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 17416 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 17328 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 17366 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 17336 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17446 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17361 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17342 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17567 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 17372 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 17511 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17368 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17385 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 17347 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17461 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17279 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 17584 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17341 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 17452 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 17464 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17485 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 17492 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 17273 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 17376 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 17481 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 17343 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 17283 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 17384 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 17319 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 17297 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17313 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 17370 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17428 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17483 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17551 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17350 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17431 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 17441 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 17425 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17412 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#north: North::Low,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 17459 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17568 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 17393 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 17326 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 17352 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17409 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17552 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17408 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17546 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 17518 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 17510 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17274 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17424 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 17422 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17423 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 17436 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17442 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17503 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 17577 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17403 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17527 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 17276 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 17455 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 17417 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 17435 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17460 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 17438 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 17404 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 17512 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17522 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17308 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17411 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 17526 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 17270 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 17383 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17415 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 17353 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17521 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17290 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 17544 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 17284 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17571 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 17371 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17465 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 17421 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 17476 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 17325 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 17439 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 17541 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 17578 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 17449 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 17316 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17354 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17494 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17558 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: true,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 17543 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17451 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 17470 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17517 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 17267 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 17529 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17286 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 17433 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17335 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17480 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 17540 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 17469 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17580 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17346 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17377 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 17355 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 17434 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17463 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 17547 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17282 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 17365 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 17390 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17287 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 17388 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 17530 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 17471 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17330 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 17573 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 17406 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 17533 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 17506 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 17440 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 17302 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17564 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 17410 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17466 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17414 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 17453 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 17524 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17264 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 17324 {
            return Some(MossyStoneBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 17559 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 17295 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 17315 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 17386 {
            return Some(MossyStoneBrickWall {
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17437 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 17549 {
            return Some(MossyStoneBrickWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 17427 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 17488 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 17534 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 17340 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 17493 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 17396 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 17407 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 17266 {
            return Some(MossyStoneBrickWall {
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 17395 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 17528 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 17391 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 17535 {
            return Some(MossyStoneBrickWall {
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 17472 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 17367 {
            return Some(MossyStoneBrickWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 17362 {
            return Some(MossyStoneBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
            });
        }
        if state_id == 17301 {
            return Some(MossyStoneBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 17306 {
            return Some(MossyStoneBrickWall {
                r#north: North::Low,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 17548 {
            return Some(MossyStoneBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 17337 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 17397 {
            return Some(MossyStoneBrickWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 17468 {
            return Some(MossyStoneBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 17323 {
            return Some(MossyStoneBrickWall {
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 17329 {
            return Some(MossyStoneBrickWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 17496 {
            return Some(MossyStoneBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        return None;
    }
}

