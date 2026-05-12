use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SandstoneWall {
    pub up: bool,
    pub r#south: South,
    pub r#east: East,
    pub r#north: North,
    pub waterlogged: bool,
    pub r#west: West,
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

impl BlockState for SandstoneWall {
    fn to_id(self) -> i32 {
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#waterlogged == false { return 19661; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#north == North::Tall { return 19608; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#up == false { return 19720; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 19779; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#up == false { return 19782; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 19621; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == true { return 19689; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 19730; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 19740; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 19794; }
        if block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 19819; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Tall { return 19846; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 19851; }
        if block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::None { return 19542; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 19631; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false { return 19776; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#north == North::None { return 19651; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#north == North::Tall { return 19726; }
        if block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#north == North::None { return 19775; }
        if block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 19620; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::None { return 19638; }
        if block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 19801; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == true { return 19617; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::None { return 19727; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::None { return 19814; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Tall { return 19607; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::None { return 19787; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#north == North::Low { return 19809; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 19604; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 19599; }
        if block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Tall { return 19772; }
        if block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#up == false { return 19791; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true { return 19826; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall { return 19840; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 19845; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 19682; }
        if block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == true { return 19652; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 19658; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#east == East::Tall { return 19771; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall { return 19804; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == false { return 19723; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Low { return 19587; }
        if block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == true { return 19752; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#west == West::None { return 19595; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false { return 19758; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 19764; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 19759; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == false { return 19783; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false { return 19543; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 19799; }
        if block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#north == North::Low { return 19570; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 19844; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Tall { return 19823; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == true { return 19583; }
        if block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::None { return 19755; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::None { return 19684; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Tall { return 19666; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::Tall { return 19774; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#up == true { return 19657; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true { return 19646; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Low { return 19647; }
        if block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#up == true { return 19749; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 19701; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == true { return 19667; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#east == East::Low { return 19645; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#south == South::Low { return 19768; }
        if block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == true { return 19623; }
        if block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 19790; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == true { return 19760; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#north == North::Tall { return 19609; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::None { return 19626; }
        if block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == false { return 19781; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true { return 19641; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::None { return 19640; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::None { return 19712; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None && block_state.r#east == East::Low { return 19716; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false { return 19687; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 19633; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::Low { return 19766; }
        if block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#south == South::Low { return 19797; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#north == North::Low { return 19698; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 19558; }
        if block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Tall { return 19672; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 19705; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#south == South::Tall { return 19746; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 19572; }
        if block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true { return 19556; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Tall { return 19745; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 19855; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::None { return 19564; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#west == West::Tall { return 19735; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Tall { return 19606; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#up == true { return 19665; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == true { return 19789; }
        if block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None { return 19636; }
        if block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#west == West::None { return 19664; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#north == North::Low { return 19706; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 19808; }
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 19562; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 19567; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false { return 19825; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#west == West::None { return 19841; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low { return 19579; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::None && block_state.r#east == East::None { return 19551; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::None { return 19627; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 19590; }
        if block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#up == true { return 19547; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#south == South::None && block_state.r#north == North::Low { return 19686; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 19648; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#up == false { return 19780; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == false { return 19770; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::Low { return 19536; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 19724; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Low { return 19714; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 19854; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::None { return 19820; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 19557; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 19696; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 19816; }
        if block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == true { return 19725; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#waterlogged == true { return 19815; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::None { return 19778; }
        if block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::Low { return 19734; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#south == South::None { return 19533; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low { return 19788; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#west == West::None { return 19541; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#west == West::None { return 19625; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true { return 19822; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 19703; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::None { return 19561; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false { return 19613; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::None { return 19643; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#east == East::Tall { return 19853; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 19852; }
        if block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#north == North::Low { return 19568; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == false { return 19733; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Low { return 19669; }
        if block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#up == false && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::None { return 19550; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::Low { return 19548; }
        if block_state.r#west == West::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#north == North::None { return 19565; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#south == South::Low { return 19663; }
        if block_state.r#up == true && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 19546; }
        if block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#south == South::None { return 19713; }
        if block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#west == West::Tall { return 19660; }
        if block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 19737; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::None { return 19619; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#west == West::Low { return 19821; }
        if block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 19731; }
        if block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::None && block_state.r#up == true && block_state.r#waterlogged == true { return 19736; }
        if block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Tall { return 19704; }
        if block_state.r#south == South::Tall && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#up == false { return 19850; }
        if block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#up == false { return 19670; }
        if block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 19554; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::None { return 19676; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#up == true && block_state.r#south == South::Tall { return 19629; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None { return 19560; }
        if block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low && block_state.r#up == false { return 19662; }
        if block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#west == West::Tall { return 19603; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 19747; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#west == West::Tall { return 19534; }
        if block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::Tall { return 19835; }
        if block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#west == West::None { return 19829; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#east == East::None { return 19581; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#west == West::None { return 19802; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Low { return 19786; }
        if block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Tall { return 19593; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#west == West::None { return 19637; }
        if block_state.r#up == true && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#east == East::Low && block_state.r#waterlogged == true { return 19642; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 19710; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#waterlogged == false { return 19842; }
        if block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None { return 19585; }
        if block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#east == East::None { return 19576; }
        if block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#south == South::Low { return 19688; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == true { return 19535; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#up == true { return 19717; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Low { return 19798; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 19577; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Tall { return 19830; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 19566; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#north == North::Low { return 19594; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true { return 19753; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Low { return 19708; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#up == true { return 19811; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 19654; }
        if block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::Low { return 19683; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 19690; }
        if block_state.r#north == North::Low && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#waterlogged == true { return 19694; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 19602; }
        if block_state.r#up == true && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#east == East::Low { return 19729; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == false { return 19538; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Low { return 19697; }
        if block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 19839; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Low && block_state.r#up == true { return 19653; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::Low { return 19763; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#north == North::Tall { return 19837; }
        if block_state.r#north == North::Tall && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 19743; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == true { return 19677; }
        if block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low { return 19679; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 19707; }
        if block_state.r#up == true && block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 19656; }
        if block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#west == West::Tall { return 19678; }
        if block_state.r#east == East::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Low { return 19807; }
        if block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#waterlogged == false { return 19655; }
        if block_state.r#east == East::Low && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false { return 19732; }
        if block_state.r#north == North::None && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#up == true { return 19761; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Low && block_state.r#up == true { return 19836; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 19563; }
        if block_state.r#south == South::None && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#up == false { return 19795; }
        if block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#south == South::Low { return 19695; }
        if block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#east == East::None { return 19634; }
        if block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#west == West::Low { return 19671; }
        if block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#west == West::Low { return 19767; }
        if block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Low { return 19785; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 19738; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall { return 19762; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#west == West::Tall { return 19813; }
        if block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#south == South::None && block_state.r#waterlogged == false { return 19831; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == false { return 19610; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#south == South::None { return 19792; }
        if block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#south == South::None { return 19644; }
        if block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#up == false && block_state.r#north == North::Tall { return 19722; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true { return 19622; }
        if block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false { return 19806; }
        if block_state.r#west == West::Low && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 19569; }
        if block_state.r#up == false && block_state.r#east == East::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Low && block_state.r#south == South::Low { return 19803; }
        if block_state.r#up == true && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false { return 19681; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#west == West::Low { return 19596; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::None && block_state.r#east == East::Tall { return 19777; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#north == North::Tall && block_state.r#up == true { return 19824; }
        if block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::None && block_state.r#up == true && block_state.r#waterlogged == false { return 19571; }
        if block_state.r#west == West::Tall && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true { return 19537; }
        if block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == true { return 19549; }
        if block_state.r#south == South::Tall && block_state.r#west == West::Low && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == true { return 19668; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 19639; }
        if block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#east == East::Low { return 19715; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::Low && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true { return 19574; }
        if block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 19702; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#south == South::Tall { return 19810; }
        if block_state.r#west == West::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true { return 19834; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None { return 19817; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#up == false { return 19598; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low { return 19848; }
        if block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 19847; }
        if block_state.r#north == North::None && block_state.r#west == West::Tall && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true { return 19540; }
        if block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall { return 19711; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::None { return 19616; }
        if block_state.r#north == North::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None { return 19553; }
        if block_state.r#up == true && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#waterlogged == true && block_state.r#east == East::None { return 19582; }
        if block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low { return 19584; }
        if block_state.r#east == East::Low && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#up == true { return 19692; }
        if block_state.r#east == East::Low && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::Low && block_state.r#waterlogged == false { return 19693; }
        if block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == true { return 19586; }
        if block_state.r#waterlogged == false && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::Low { return 19800; }
        if block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Low && block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::None { return 19544; }
        if block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == false && block_state.r#waterlogged == false { return 19589; }
        if block_state.r#waterlogged == true && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#west == West::None && block_state.r#up == true { return 19532; }
        if block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 19618; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#north == North::None && block_state.r#up == true && block_state.r#east == East::Tall { return 19751; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#up == false { return 19673; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::Low { return 19605; }
        if block_state.r#south == South::Low && block_state.r#east == East::None && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#north == North::None && block_state.r#waterlogged == true { return 19545; }
        if block_state.r#north == North::Tall && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true { return 19611; }
        if block_state.r#east == East::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#south == South::Low { return 19580; }
        if block_state.r#north == North::Tall && block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false && block_state.r#south == South::None { return 19612; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#up == true { return 19750; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#north == North::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 19675; }
        if block_state.r#south == South::Tall && block_state.r#east == East::Low && block_state.r#north == North::Tall && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false { return 19744; }
        if block_state.r#north == North::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#west == West::None { return 19748; }
        if block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::None { return 19709; }
        if block_state.r#north == North::None && block_state.r#east == East::Tall && block_state.r#west == West::Low && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#waterlogged == true { return 19773; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#up == true { return 19812; }
        if block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#north == North::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#up == false { return 19600; }
        if block_state.r#east == East::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#north == North::None && block_state.r#south == South::Tall { return 19674; }
        if block_state.r#waterlogged == false && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#north == North::Low && block_state.r#south == South::Low { return 19699; }
        if block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#west == West::Tall && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#north == North::Tall { return 19741; }
        if block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Low { return 19721; }
        if block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#north == North::Low { return 19597; }
        if block_state.r#north == North::None && block_state.r#up == true && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 19559; }
        if block_state.r#north == North::Tall && block_state.r#west == West::None && block_state.r#south == South::Tall && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#east == East::Low { return 19742; }
        if block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#north == North::Tall { return 19632; }
        if block_state.r#north == North::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true { return 19739; }
        if block_state.r#west == West::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#east == East::None && block_state.r#south == South::Low && block_state.r#up == false { return 19555; }
        if block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#waterlogged == false && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#south == South::None { return 19680; }
        if block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#north == North::Tall { return 19635; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#east == East::Low && block_state.r#up == true && block_state.r#south == South::Tall { return 19700; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#waterlogged == true && block_state.r#south == South::Tall && block_state.r#east == East::None && block_state.r#west == West::None { return 19628; }
        if block_state.r#waterlogged == true && block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Low { return 19575; }
        if block_state.r#west == West::None && block_state.r#waterlogged == false && block_state.r#north == North::None && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::None { return 19649; }
        if block_state.r#up == false && block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#west == West::None { return 19754; }
        if block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#south == South::None { return 19756; }
        if block_state.r#west == West::Low && block_state.r#east == East::None && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == true && block_state.r#north == North::None { return 19539; }
        if block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Low && block_state.r#south == South::None { return 19578; }
        if block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#east == East::None { return 19614; }
        if block_state.r#north == North::None && block_state.r#waterlogged == true && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#west == West::Low && block_state.r#south == South::Low { return 19659; }
        if block_state.r#west == West::None && block_state.r#south == South::None && block_state.r#east == East::Tall && block_state.r#north == North::Low && block_state.r#up == false && block_state.r#waterlogged == false { return 19793; }
        if block_state.r#south == South::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#west == West::Tall && block_state.r#north == North::Tall { return 19615; }
        if block_state.r#north == North::None && block_state.r#west == West::None && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#waterlogged == false { return 19757; }
        if block_state.r#up == false && block_state.r#east == East::None && block_state.r#north == North::Tall && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall { return 19624; }
        if block_state.r#east == East::Low && block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#west == West::Low && block_state.r#up == false && block_state.r#north == North::None { return 19650; }
        if block_state.r#waterlogged == false && block_state.r#west == West::None && block_state.r#east == East::None && block_state.r#up == false && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 19601; }
        if block_state.r#west == West::None && block_state.r#north == North::Tall && block_state.r#up == false && block_state.r#east == East::Low && block_state.r#south == South::None && block_state.r#waterlogged == true { return 19718; }
        if block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#waterlogged == false && block_state.r#up == false { return 19769; }
        if block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#up == true && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#waterlogged == false { return 19691; }
        if block_state.r#west == West::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#south == South::None && block_state.r#up == false && block_state.r#north == North::Tall { return 19827; }
        if block_state.r#north == North::Low && block_state.r#west == West::None && block_state.r#south == South::Low && block_state.r#up == true && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 19796; }
        if block_state.r#waterlogged == false && block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Low { return 19728; }
        if block_state.r#west == West::Tall && block_state.r#up == true && block_state.r#waterlogged == false && block_state.r#east == East::Tall && block_state.r#north == North::None && block_state.r#south == South::Low { return 19765; }
        if block_state.r#south == South::None && block_state.r#up == true && block_state.r#east == East::None && block_state.r#north == North::Low && block_state.r#waterlogged == false && block_state.r#west == West::Tall { return 19573; }
        if block_state.r#north == North::Tall && block_state.r#south == South::None && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::Tall && block_state.r#up == false { return 19828; }
        if block_state.r#waterlogged == true && block_state.r#north == North::Tall && block_state.r#east == East::Tall && block_state.r#up == true && block_state.r#west == West::None && block_state.r#south == South::Low { return 19832; }
        if block_state.r#up == false && block_state.r#north == North::Low && block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#west == West::Tall && block_state.r#east == East::None { return 19588; }
        if block_state.r#north == North::Tall && block_state.r#up == true && block_state.r#west == West::Low && block_state.r#south == South::Low && block_state.r#east == East::Tall && block_state.r#waterlogged == true { return 19833; }
        if block_state.r#east == East::None && block_state.r#north == North::None && block_state.r#south == South::Low && block_state.r#up == false && block_state.r#west == West::Tall && block_state.r#waterlogged == true { return 19552; }
        if block_state.r#waterlogged == false && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#west == West::None && block_state.r#up == false && block_state.r#north == North::Low { return 19685; }
        if block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#east == East::None && block_state.r#west == West::Tall && block_state.r#south == South::Tall && block_state.r#north == North::Tall { return 19630; }
        if block_state.r#waterlogged == true && block_state.r#south == South::None && block_state.r#east == East::Low && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::Low { return 19719; }
        if block_state.r#south == South::Low && block_state.r#waterlogged == true && block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#north == North::Tall && block_state.r#west == West::None { return 19838; }
        if block_state.r#east == East::Tall && block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::Tall && block_state.r#up == false { return 19843; }
        if block_state.r#north == North::Low && block_state.r#east == East::Tall && block_state.r#west == West::None && block_state.r#waterlogged == true && block_state.r#up == true && block_state.r#south == South::None { return 19784; }
        if block_state.r#east == East::Tall && block_state.r#up == false && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#west == West::None && block_state.r#north == North::Low { return 19805; }
        if block_state.r#north == North::Tall && block_state.r#waterlogged == false && block_state.r#south == South::Tall && block_state.r#east == East::Tall && block_state.r#west == West::Tall && block_state.r#up == true { return 19849; }
        if block_state.r#east == East::None && block_state.r#waterlogged == true && block_state.r#west == West::None && block_state.r#up == true && block_state.r#south == South::Tall && block_state.r#north == North::Low { return 19592; }
        if block_state.r#east == East::None && block_state.r#waterlogged == false && block_state.r#south == South::Low && block_state.r#north == North::Low && block_state.r#west == West::Tall && block_state.r#up == false { return 19591; }
        if block_state.r#north == North::Low && block_state.r#up == false && block_state.r#west == West::Low && block_state.r#east == East::Tall && block_state.r#south == South::Tall && block_state.r#waterlogged == false { return 19818; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 19661 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19608 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 19720 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 19779 {
            return Some(SandstoneWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19782 {
            return Some(SandstoneWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 19621 {
            return Some(SandstoneWall {
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19689 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 19730 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 19740 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 19794 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19819 {
            return Some(SandstoneWall {
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19846 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19851 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19542 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 19631 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 19776 {
            return Some(SandstoneWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19651 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19726 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 19775 {
            return Some(SandstoneWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 19620 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19638 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 19801 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19617 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 19727 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 19814 {
            return Some(SandstoneWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19607 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 19787 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 19809 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 19604 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19599 {
            return Some(SandstoneWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19772 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19791 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 19826 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19840 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19845 {
            return Some(SandstoneWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19682 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 19652 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19658 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 19771 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 19804 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19723 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 19587 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 19752 {
            return Some(SandstoneWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19595 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 19758 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19764 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 19759 {
            return Some(SandstoneWall {
                r#up: false,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19783 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 19543 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19799 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19570 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 19844 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19823 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 19583 {
            return Some(SandstoneWall {
                r#south: South::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 19755 {
            return Some(SandstoneWall {
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 19684 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 19666 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 19774 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 19657 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 19646 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19647 {
            return Some(SandstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 19749 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19701 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19667 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 19645 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 19768 {
            return Some(SandstoneWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 19623 {
            return Some(SandstoneWall {
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19790 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#south: South::None,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 19760 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 19609 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 19626 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 19781 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 19641 {
            return Some(SandstoneWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19640 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 19712 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#up: true,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 19716 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 19687 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19633 {
            return Some(SandstoneWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19766 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 19797 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19698 {
            return Some(SandstoneWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 19558 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19672 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 19705 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19746 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19572 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19556 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19745 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19855 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19564 {
            return Some(SandstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 19735 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19606 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19665 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 19789 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 19636 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 19664 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19706 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 19808 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19562 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19567 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19825 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19841 {
            return Some(SandstoneWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19579 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 19551 {
            return Some(SandstoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 19627 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 19590 {
            return Some(SandstoneWall {
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19547 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 19686 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 19648 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 19780 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 19770 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19536 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 19724 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19714 {
            return Some(SandstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 19854 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19820 {
            return Some(SandstoneWall {
                r#up: true,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19557 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19696 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19816 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19725 {
            return Some(SandstoneWall {
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19815 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19778 {
            return Some(SandstoneWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 19734 {
            return Some(SandstoneWall {
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19533 {
            return Some(SandstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 19788 {
            return Some(SandstoneWall {
                r#up: true,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 19541 {
            return Some(SandstoneWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 19625 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19822 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 19703 {
            return Some(SandstoneWall {
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 19561 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 19613 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 19643 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 19853 {
            return Some(SandstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19852 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 19568 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 19733 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19669 {
            return Some(SandstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 19550 {
            return Some(SandstoneWall {
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 19548 {
            return Some(SandstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 19565 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 19663 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 19546 {
            return Some(SandstoneWall {
                r#up: true,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19713 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19660 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 19737 {
            return Some(SandstoneWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19619 {
            return Some(SandstoneWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 19821 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 19731 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 19736 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19704 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19850 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 19670 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 19554 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19676 {
            return Some(SandstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 19629 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19560 {
            return Some(SandstoneWall {
                r#up: true,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 19662 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 19603 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19747 {
            return Some(SandstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19534 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 19835 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 19829 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19581 {
            return Some(SandstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 19802 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 19786 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 19593 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 19637 {
            return Some(SandstoneWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19642 {
            return Some(SandstoneWall {
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19710 {
            return Some(SandstoneWall {
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19842 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19585 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 19576 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 19688 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19535 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 19717 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 19798 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 19577 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19830 {
            return Some(SandstoneWall {
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19566 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19594 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
            });
        }
        if state_id == 19753 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 19708 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19811 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 19654 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19683 {
            return Some(SandstoneWall {
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 19690 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19694 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#east: East::Low,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19602 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19729 {
            return Some(SandstoneWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 19538 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 19697 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 19839 {
            return Some(SandstoneWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19653 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 19763 {
            return Some(SandstoneWall {
                r#up: true,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 19837 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19743 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19677 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 19679 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 19707 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19656 {
            return Some(SandstoneWall {
                r#up: true,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 19678 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: true,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 19807 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 19655 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19732 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 19761 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 19836 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 19563 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19795 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 19695 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 19634 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 19671 {
            return Some(SandstoneWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 19767 {
            return Some(SandstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 19785 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 19738 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19762 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 19813 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 19831 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19610 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 19792 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19644 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 19722 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 19622 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19806 {
            return Some(SandstoneWall {
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 19569 {
            return Some(SandstoneWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19803 {
            return Some(SandstoneWall {
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19681 {
            return Some(SandstoneWall {
                r#up: true,
                r#east: East::Low,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19596 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 19777 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 19824 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 19571 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 19537 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 19549 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 19668 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 19639 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19715 {
            return Some(SandstoneWall {
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 19574 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19702 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19810 {
            return Some(SandstoneWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19834 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 19817 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19598 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#up: false,
            });
        }
        if state_id == 19848 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 19847 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 19540 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19711 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19616 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 19553 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 19582 {
            return Some(SandstoneWall {
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 19584 {
            return Some(SandstoneWall {
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 19692 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 19693 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 19586 {
            return Some(SandstoneWall {
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 19800 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19544 {
            return Some(SandstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 19589 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19532 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
                r#east: East::None,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 19618 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19751 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 19673 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
            });
        }
        if state_id == 19605 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 19545 {
            return Some(SandstoneWall {
                r#south: South::Low,
                r#east: East::None,
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19611 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 19580 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 19612 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 19750 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 19675 {
            return Some(SandstoneWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19744 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 19748 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 19709 {
            return Some(SandstoneWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 19773 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19812 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 19600 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 19674 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 19699 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19741 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 19721 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 19597 {
            return Some(SandstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19559 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 19742 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 19632 {
            return Some(SandstoneWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 19739 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 19555 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 19680 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 19635 {
            return Some(SandstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 19700 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 19628 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 19575 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 19649 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 19754 {
            return Some(SandstoneWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19756 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 19539 {
            return Some(SandstoneWall {
                r#west: West::Low,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 19578 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 19614 {
            return Some(SandstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 19659 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 19793 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19615 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19757 {
            return Some(SandstoneWall {
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 19624 {
            return Some(SandstoneWall {
                r#up: false,
                r#east: East::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 19650 {
            return Some(SandstoneWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 19601 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19718 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 19769 {
            return Some(SandstoneWall {
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 19691 {
            return Some(SandstoneWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 19827 {
            return Some(SandstoneWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 19796 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19728 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 19765 {
            return Some(SandstoneWall {
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 19573 {
            return Some(SandstoneWall {
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 19828 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 19832 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
            });
        }
        if state_id == 19588 {
            return Some(SandstoneWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 19833 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19552 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 19685 {
            return Some(SandstoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::None,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 19630 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 19719 {
            return Some(SandstoneWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 19838 {
            return Some(SandstoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 19843 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 19784 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 19805 {
            return Some(SandstoneWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 19849 {
            return Some(SandstoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 19592 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 19591 {
            return Some(SandstoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 19818 {
            return Some(SandstoneWall {
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

