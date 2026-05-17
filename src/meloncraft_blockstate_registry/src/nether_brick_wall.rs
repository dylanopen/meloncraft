use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetherBrickWall {
    pub r#east: East,
    pub r#west: West,
    pub r#north: North,
    pub up: bool,
    pub waterlogged: bool,
    pub r#south: South,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum South {
    None,
    Low,
    Tall,
}

impl BlockState for NetherBrickWall {
    fn to_id(&self) -> i32 {
        if self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#up == false && self.r#east == East::Low { return 18711; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low { return 18618; }
        if self.r#south == South::None && self.r#up == false && self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 18604; }
        if self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#east == East::None { return 18631; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::Low { return 18686; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::None && self.r#up == true && self.r#north == North::None { return 18692; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Tall { return 18694; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low { return 18682; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::Low && self.r#up == true { return 18732; }
        if self.r#up == false && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == true { return 18880; }
        if self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Tall && self.r#up == true { return 18649; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall { return 18745; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#up == true && self.r#west == West::None { return 18743; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#west == West::None && self.r#waterlogged == false && self.r#up == false && self.r#south == South::Tall { return 18845; }
        if self.r#up == false && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low { return 18726; }
        if self.r#south == South::Low && self.r#west == West::None && self.r#up == false && self.r#east == East::None && self.r#north == North::None && self.r#waterlogged == false { return 18581; }
        if self.r#west == West::Tall && self.r#up == false && self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::None && self.r#waterlogged == true { return 18820; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::Low && self.r#up == true && self.r#south == South::Low { return 18721; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 18796; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low && self.r#up == false && self.r#east == East::Tall && self.r#north == North::Tall { return 18869; }
        if self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::None && self.r#north == North::Tall { return 18866; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::Low && self.r#up == false { return 18846; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::None && self.r#up == true && self.r#north == North::Low { return 18815; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#up == true { return 18731; }
        if self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::None && self.r#up == false && self.r#east == East::Tall { return 18859; }
        if self.r#north == North::None && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::None { return 18568; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Low && self.r#up == true { return 18685; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#waterlogged == false && self.r#up == true && self.r#east == East::Tall && self.r#south == South::None { return 18853; }
        if self.r#west == West::None && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::None { return 18854; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#up == true && self.r#north == North::Low { return 18613; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 18580; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#up == true && self.r#west == West::None { return 18599; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false { return 18691; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::None && self.r#up == false && self.r#north == North::Low && self.r#west == West::None { return 18629; }
        if self.r#east == East::Low && self.r#up == true && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall { return 18670; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall && self.r#up == false && self.r#north == North::Low { return 18842; }
        if self.r#up == false && self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Tall { return 18655; }
        if self.r#south == South::Low && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == false { return 18762; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#west == West::Tall && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false { return 18799; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == true { return 18873; }
        if self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#up == true { return 18792; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true { return 18716; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Low && self.r#up == false && self.r#north == North::Tall { return 18870; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::Low { return 18612; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::Tall && self.r#up == false { return 18640; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 18772; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false { return 18665; }
        if self.r#up == false && self.r#south == South::Tall && self.r#east == East::None && self.r#west == West::Low && self.r#north == North::Low && self.r#waterlogged == true { return 18627; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::None && self.r#up == true { return 18851; }
        if self.r#west == West::None && self.r#up == true && self.r#south == South::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::None { return 18632; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::Low && self.r#north == North::None && self.r#up == false { return 18797; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#up == false && self.r#north == North::Tall && self.r#east == East::Tall { return 18855; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#up == true { return 18656; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Tall { return 18881; }
        if self.r#east == East::None && self.r#up == true && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::Tall { return 18659; }
        if self.r#east == East::Low && self.r#up == true && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == false { return 18683; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == false && self.r#west == West::None && self.r#north == North::Tall { return 18746; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true && self.r#east == East::Low { return 18757; }
        if self.r#waterlogged == false && self.r#up == false && self.r#north == North::Low && self.r#west == West::None && self.r#east == East::Tall && self.r#south == South::None { return 18821; }
        if self.r#west == West::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == true && self.r#east == East::None { return 18636; }
        if self.r#up == false && self.r#south == South::None && self.r#west == West::Low && self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::None { return 18678; }
        if self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::Tall { return 18771; }
        if self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::None && self.r#up == true && self.r#waterlogged == true { return 18801; }
        if self.r#up == false && self.r#west == West::None && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == true { return 18602; }
        if self.r#up == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#west == West::Tall && self.r#south == South::None && self.r#waterlogged == true { return 18748; }
        if self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Low { return 18623; }
        if self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == false && self.r#east == East::None { return 18628; }
        if self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 18810; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None { return 18848; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None && self.r#north == North::Tall && self.r#up == false { return 18653; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Low && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == false { return 18719; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Tall { return 18879; }
        if self.r#west == West::Low && self.r#east == East::None && self.r#up == true && self.r#south == South::Tall && self.r#north == North::Low && self.r#waterlogged == false { return 18624; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::None && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Tall { return 18634; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::None && self.r#up == true && self.r#west == West::Low && self.r#north == North::None { return 18561; }
        if self.r#up == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == false { return 18774; }
        if self.r#south == South::Low && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == true && self.r#west == West::Low { return 18828; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Low { return 18648; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low { return 18735; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#up == true && self.r#west == West::None && self.r#east == East::None && self.r#south == South::None { return 18596; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == true && self.r#west == West::None && self.r#north == North::Tall { return 18860; }
        if self.r#east == East::None && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::None { return 18626; }
        if self.r#north == North::None && self.r#up == false && self.r#south == South::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::None { return 18593; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::None && self.r#up == true && self.r#south == South::None { return 18565; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#waterlogged == true && self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == false { return 18664; }
        if self.r#up == true && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::None { return 18706; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::None { return 18783; }
        if self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true && self.r#up == false { return 18784; }
        if self.r#west == West::None && self.r#south == South::Tall && self.r#up == true && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::None { return 18584; }
        if self.r#south == South::Tall && self.r#west == West::Tall && self.r#up == true && self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == false { return 18697; }
        if self.r#east == East::None && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall { return 18577; }
        if self.r#waterlogged == true && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Low && self.r#west == West::Tall && self.r#north == North::Tall { return 18868; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low { return 18834; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Tall && self.r#west == West::None && self.r#up == true { return 18740; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == false && self.r#north == North::None { return 18807; }
        if self.r#south == South::Low && self.r#east == East::Low && self.r#west == West::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true { return 18752; }
        if self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::Low { return 18594; }
        if self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == true { return 18826; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::None && self.r#west == West::Low && self.r#up == false && self.r#east == East::None { return 18567; }
        if self.r#north == North::None && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall { return 18595; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == false && self.r#west == West::Tall { return 18739; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 18808; }
        if self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::Low { return 18738; }
        if self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::None && self.r#west == West::Low { return 18669; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#up == true && self.r#north == North::Low && self.r#east == East::None { return 18620; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false { return 18794; }
        if self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#up == false { return 18630; }
        if self.r#up == true && self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::None { return 18684; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#up == true { return 18717; }
        if self.r#up == false && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None { return 18758; }
        if self.r#south == South::None && self.r#up == true && self.r#north == North::None && self.r#west == West::None && self.r#east == East::Tall && self.r#waterlogged == true { return 18776; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#waterlogged == true { return 18856; }
        if self.r#up == true && self.r#east == East::None && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == true { return 18573; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall { return 18823; }
        if self.r#west == West::Low && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Low { return 18798; }
        if self.r#up == false && self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None { return 18590; }
        if self.r#west == West::None && self.r#north == North::None && self.r#up == false && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Low { return 18674; }
        if self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low { return 18699; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Tall && self.r#up == true && self.r#north == North::None { return 18586; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == true { return 18812; }
        if self.r#up == true && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Low { return 18753; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == true { return 18652; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::Low && self.r#up == true { return 18769; }
        if self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::None && self.r#up == false { return 18569; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == false { return 18883; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#south == South::None && self.r#west == West::None && self.r#waterlogged == false { return 18677; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#south == South::Low && self.r#up == false && self.r#west == West::None && self.r#east == East::Low { return 18725; }
        if self.r#west == West::None && self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::Tall { return 18857; }
        if self.r#up == false && self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::Tall { return 18867; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::None && self.r#east == East::Tall { return 18817; }
        if self.r#south == South::None && self.r#east == East::None && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low { return 18600; }
        if self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low { return 18835; }
        if self.r#waterlogged == false && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::None && self.r#up == true && self.r#west == West::Low { return 18672; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::Low && self.r#up == false && self.r#west == West::Low && self.r#south == South::None { return 18714; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::None { return 18609; }
        if self.r#up == true && self.r#west == West::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Tall { return 18741; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::None && self.r#up == false && self.r#north == North::None && self.r#waterlogged == true { return 18675; }
        if self.r#south == South::None && self.r#north == North::Tall && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 18637; }
        if self.r#up == false && self.r#north == North::Low && self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None { return 18605; }
        if self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Tall { return 18695; }
        if self.r#east == East::Tall && self.r#up == true && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low { return 18837; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 18730; }
        if self.r#up == true && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None && self.r#east == East::Low { return 18680; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#up == true && self.r#north == North::Low && self.r#south == South::None { return 18707; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::None && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == true { return 18645; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Tall { return 18701; }
        if self.r#north == North::Low && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true && self.r#south == South::Tall { return 18844; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::None && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false { return 18582; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Tall { return 18877; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#waterlogged == true && self.r#up == false && self.r#east == East::None && self.r#south == South::Tall { return 18592; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::None && self.r#up == true { return 18658; }
        if self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None && self.r#east == East::None { return 18635; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#up == false { return 18832; }
        if self.r#east == East::None && self.r#south == South::None && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Low && self.r#waterlogged == true { return 18598; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false { return 18775; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == false && self.r#north == North::Low { return 18847; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == true && self.r#north == North::Tall { return 18876; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#up == true { return 18608; }
        if self.r#south == South::Tall && self.r#up == true && self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall { return 18805; }
        if self.r#east == East::Tall && self.r#up == true && self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::None { return 18827; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true && self.r#west == West::Low && self.r#east == East::None { return 18585; }
        if self.r#up == true && self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::None { return 18816; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == true && self.r#east == East::None { return 18574; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall { return 18610; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#west == West::Low && self.r#east == East::None && self.r#waterlogged == true { return 18579; }
        if self.r#up == true && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == false { return 18840; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 18709; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#up == true && self.r#east == East::Tall && self.r#west == West::Tall && self.r#waterlogged == false { return 18829; }
        if self.r#west == West::None && self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#up == true { return 18779; }
        if self.r#west == West::None && self.r#east == East::Tall && self.r#up == false && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == true { return 18878; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::None { return 18696; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::Low && self.r#up == true { return 18825; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == false && self.r#north == North::Low { return 18843; }
        if self.r#waterlogged == true && self.r#up == false && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::Low && self.r#west == West::Low { return 18615; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::Tall { return 18838; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Tall && self.r#south == South::None && self.r#east == East::Tall { return 18850; }
        if self.r#south == South::Low && self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::None { return 18761; }
        if self.r#up == true && self.r#south == South::Tall && self.r#west == West::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::None { return 18588; }
        if self.r#south == South::Low && self.r#up == false && self.r#east == East::None && self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::Low { return 18614; }
        if self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::None && self.r#up == false && self.r#west == West::Tall { return 18715; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::None && self.r#up == false && self.r#west == West::Tall && self.r#south == South::Low { return 18583; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::Tall && self.r#west == West::Low && self.r#up == false { return 18831; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#up == true && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Tall { return 18728; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::Low && self.r#up == true { return 18622; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#east == East::Tall { return 18839; }
        if self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Low && self.r#south == South::Low { return 18720; }
        if self.r#north == North::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::None && self.r#up == true { return 18601; }
        if self.r#east == East::None && self.r#south == South::None && self.r#north == North::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 18566; }
        if self.r#east == East::None && self.r#up == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == true { return 18657; }
        if self.r#east == East::Low && self.r#up == true && self.r#south == South::None && self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::None { return 18668; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#waterlogged == true && self.r#up == true && self.r#east == East::Low && self.r#west == West::Tall { return 18718; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#up == false && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low { return 18786; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == true && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::None { return 18803; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::Low { return 18660; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#east == East::Tall && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == true { return 18819; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == true { return 18875; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall { return 18882; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::None { return 18633; }
        if self.r#east == East::None && self.r#west == West::Low && self.r#up == false && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Tall { return 18642; }
        if self.r#up == true && self.r#west == West::Low && self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::Low { return 18708; }
        if self.r#east == East::Tall && self.r#up == false && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::None && self.r#waterlogged == false { return 18811; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == false { return 18806; }
        if self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::None && self.r#waterlogged == true && self.r#up == false { return 18712; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::Low && self.r#up == true && self.r#west == West::Low && self.r#east == East::None { return 18576; }
        if self.r#up == true && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::None && self.r#north == North::Low && self.r#waterlogged == true { return 18704; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false && self.r#south == South::None { return 18603; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#up == false && self.r#south == South::None && self.r#east == East::None && self.r#north == North::Tall { return 18639; }
        if self.r#east == East::Low && self.r#up == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Low { return 18759; }
        if self.r#north == North::Tall && self.r#up == false && self.r#east == East::None && self.r#west == West::Low && self.r#south == South::Tall && self.r#waterlogged == false { return 18666; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::Low && self.r#up == false && self.r#east == East::Tall && self.r#west == West::None { return 18818; }
        if self.r#up == false && self.r#east == East::Tall && self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low { return 18871; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == true && self.r#east == East::Tall && self.r#south == South::Low { return 18789; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == false && self.r#west == West::None && self.r#south == South::None { return 18641; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::None { return 18787; }
        if self.r#south == South::None && self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Low { return 18744; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low { return 18747; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::Tall { return 18750; }
        if self.r#waterlogged == true && self.r#up == false && self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::None && self.r#east == East::Low { return 18676; }
        if self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == true { return 18754; }
        if self.r#north == North::Tall && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false { return 18864; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == true { return 18742; }
        if self.r#north == North::Low && self.r#up == true && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::None && self.r#east == East::Tall { return 18824; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#east == East::Tall && self.r#west == West::None && self.r#waterlogged == true { return 18830; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::None && self.r#east == East::Tall { return 18849; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low && self.r#up == true { return 18681; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::None { return 18643; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Low && self.r#south == South::None && self.r#up == false { return 18822; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 18690; }
        if self.r#up == true && self.r#west == West::Low && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Low { return 18861; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall { return 18733; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#north == North::Low && self.r#up == true { return 18813; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::None && self.r#up == true { return 18778; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Low && self.r#up == false && self.r#west == West::None && self.r#north == North::None { return 18698; }
        if self.r#north == North::Low && self.r#east == East::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None { return 18722; }
        if self.r#east == East::Low && self.r#west == West::None && self.r#waterlogged == false && self.r#up == false && self.r#south == South::None && self.r#north == North::Tall { return 18749; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Low && self.r#up == true { return 18790; }
        if self.r#west == West::Tall && self.r#up == false && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::Low { return 18688; }
        if self.r#north == North::Low && self.r#up == false && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#west == West::Tall { return 18727; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false { return 18606; }
        if self.r#up == false && self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::Low { return 18770; }
        if self.r#up == false && self.r#north == North::Tall && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Low && self.r#waterlogged == false { return 18763; }
        if self.r#east == East::None && self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false { return 18654; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::None && self.r#west == West::None && self.r#up == true && self.r#north == North::None { return 18572; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::None { return 18575; }
        if self.r#south == South::None && self.r#east == East::Low && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::Low { return 18713; }
        if self.r#up == false && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 18700; }
        if self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::None && self.r#west == West::None { return 18647; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Tall && self.r#up == true && self.r#north == North::None { return 18800; }
        if self.r#south == South::Tall && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Tall { return 18766; }
        if self.r#west == West::Low && self.r#up == true && self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall { return 18804; }
        if self.r#west == West::Tall && self.r#up == false && self.r#north == North::None && self.r#east == East::None && self.r#south == South::None && self.r#waterlogged == false { return 18571; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::None && self.r#up == false && self.r#east == East::None { return 18578; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Tall && self.r#up == false { return 18667; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#west == West::Tall && self.r#north == North::Tall && self.r#waterlogged == false && self.r#up == true { return 18661; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#north == North::Low && self.r#west == West::Low { return 18723; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None && self.r#east == East::Tall && self.r#north == North::None && self.r#up == false { return 18785; }
        if self.r#up == true && self.r#south == South::Tall && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::None { return 18802; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Tall { return 18865; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Tall && self.r#waterlogged == true && self.r#up == true { return 18874; }
        if self.r#west == West::Tall && self.r#north == North::Low && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Tall { return 18841; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 18736; }
        if self.r#north == North::Low && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low { return 18737; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Low { return 18663; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::None { return 18597; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true { return 18836; }
        if self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Tall { return 18872; }
        if self.r#east == East::None && self.r#up == false && self.r#north == North::Tall && self.r#west == West::Low && self.r#south == South::Low && self.r#waterlogged == true { return 18651; }
        if self.r#west == West::None && self.r#up == true && self.r#waterlogged == false && self.r#south == South::None && self.r#north == North::None && self.r#east == East::Low { return 18671; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Low && self.r#up == true && self.r#west == West::Low { return 18756; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == true && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 18814; }
        if self.r#west == West::Low && self.r#south == South::None && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == false && self.r#up == false { return 18570; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::Low && self.r#up == false { return 18617; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::None && self.r#south == South::Tall && self.r#up == true { return 18625; }
        if self.r#south == South::Low && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None { return 18644; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == true { return 18687; }
        if self.r#waterlogged == false && self.r#up == true && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None && self.r#west == West::Low { return 18780; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::Tall { return 18863; }
        if self.r#up == true && self.r#south == South::None && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::None && self.r#west == West::None { return 18560; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::None && self.r#up == true && self.r#west == West::Low && self.r#east == East::None { return 18564; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Low { return 18724; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low { return 18765; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low && self.r#up == true { return 18791; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Low { return 18705; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#up == false && self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::Low { return 18734; }
        if self.r#waterlogged == false && self.r#up == false && self.r#north == North::None && self.r#east == East::Tall && self.r#west == West::None && self.r#south == South::Tall { return 18809; }
        if self.r#up == true && self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Low && self.r#waterlogged == false { return 18755; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::Low && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == false { return 18760; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#up == true { return 18764; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == true && self.r#east == East::Low && self.r#west == West::Low { return 18729; }
        if self.r#west == West::Tall && self.r#up == true && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == false { return 18793; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#up == false && self.r#south == South::None && self.r#east == East::Low && self.r#west == West::Tall { return 18679; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None && self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == false { return 18781; }
        if self.r#north == North::Tall && self.r#up == true && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall { return 18852; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#waterlogged == true && self.r#up == true && self.r#east == East::None && self.r#south == South::None { return 18562; }
        if self.r#up == false && self.r#east == East::None && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::Tall { return 18662; }
        if self.r#east == East::None && self.r#up == false && self.r#west == West::None && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::Tall { return 18638; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::None && self.r#up == false { return 18858; }
        if self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::None && self.r#up == false && self.r#west == West::None { return 18650; }
        if self.r#up == false && self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::None { return 18710; }
        if self.r#up == true && self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::None && self.r#west == West::Low { return 18693; }
        if self.r#west == West::None && self.r#south == South::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#up == false && self.r#north == North::None { return 18782; }
        if self.r#east == East::None && self.r#up == false && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::Tall && self.r#waterlogged == true { return 18591; }
        if self.r#east == East::Tall && self.r#up == true && self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::Low { return 18788; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::Tall && self.r#east == East::None && self.r#north == North::Low && self.r#up == false { return 18616; }
        if self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#east == East::None { return 18563; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall { return 18703; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#up == false && self.r#west == West::Tall && self.r#south == South::None { return 18751; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 18673; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Tall { return 18646; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#east == East::None && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == false { return 18607; }
        if self.r#up == false && self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::None { return 18689; }
        if self.r#east == East::Low && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall { return 18768; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#up == false && self.r#north == North::Tall && self.r#west == West::None && self.r#east == East::Low { return 18773; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Low && self.r#up == true && self.r#north == North::Low && self.r#east == East::None { return 18611; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall { return 18619; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#up == true && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Tall { return 18589; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#west == West::Low && self.r#east == East::None && self.r#up == true && self.r#waterlogged == true { return 18621; }
        if self.r#south == South::Tall && self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::None { return 18767; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#west == West::Low && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == true { return 18795; }
        if self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::None && self.r#up == true { return 18587; }
        if self.r#up == false && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Low { return 18702; }
        if self.r#south == South::Low && self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::None { return 18833; }
        if self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Tall { return 18862; }
        if self.r#up == true && self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::None { return 18777; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 18711 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 18618 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 18604 {
            return Some(NetherBrickWall {
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18631 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 18686 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 18692 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18694 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 18682 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 18732 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 18880 {
            return Some(NetherBrickWall {
                r#up: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18649 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 18745 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18743 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18845 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::Tall,
            });
        }
        if state_id == 18726 {
            return Some(NetherBrickWall {
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 18581 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18820 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18721 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 18796 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18869 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18866 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18846 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 18815 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 18731 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 18859 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#east: East::Tall,
            });
        }
        if state_id == 18568 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18685 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18853 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18854 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18613 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 18580 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18599 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 18691 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18629 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 18670 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18842 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 18655 {
            return Some(NetherBrickWall {
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18762 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18799 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18873 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18792 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 18716 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18870 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18612 {
            return Some(NetherBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 18640 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 18772 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18665 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18627 {
            return Some(NetherBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18851 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 18632 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#up: true,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 18797 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 18855 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18656 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 18881 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 18659 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#up: true,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18683 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18746 {
            return Some(NetherBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18757 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 18821 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18636 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
                r#east: East::None,
            });
        }
        if state_id == 18678 {
            return Some(NetherBrickWall {
                r#up: false,
                r#south: South::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 18771 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18801 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18602 {
            return Some(NetherBrickWall {
                r#up: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18748 {
            return Some(NetherBrickWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18623 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 18628 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 18810 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18848 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18653 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18719 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18879 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18624 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18634 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18561 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 18774 {
            return Some(NetherBrickWall {
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18828 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 18648 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18735 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18596 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#west: West::None,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 18860 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18626 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18593 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18565 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 18664 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 18706 {
            return Some(NetherBrickWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 18783 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 18784 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 18584 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 18697 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18577 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18868 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18834 {
            return Some(NetherBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18740 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 18807 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 18752 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18594 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 18826 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18567 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 18595 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18739 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18808 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18738 {
            return Some(NetherBrickWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 18669 {
            return Some(NetherBrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 18620 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 18794 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
            });
        }
        if state_id == 18630 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 18684 {
            return Some(NetherBrickWall {
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 18717 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18758 {
            return Some(NetherBrickWall {
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18776 {
            return Some(NetherBrickWall {
                r#south: South::None,
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18856 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18573 {
            return Some(NetherBrickWall {
                r#up: true,
                r#east: East::None,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18823 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18798 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 18590 {
            return Some(NetherBrickWall {
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 18674 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 18699 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18586 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18812 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
            });
        }
        if state_id == 18753 {
            return Some(NetherBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18652 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 18769 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 18569 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
            });
        }
        if state_id == 18883 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 18677 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18725 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 18857 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18867 {
            return Some(NetherBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 18817 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18600 {
            return Some(NetherBrickWall {
                r#south: South::None,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18835 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18672 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 18714 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 18609 {
            return Some(NetherBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 18741 {
            return Some(NetherBrickWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18675 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18637 {
            return Some(NetherBrickWall {
                r#south: South::None,
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18605 {
            return Some(NetherBrickWall {
                r#up: false,
                r#north: North::Low,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 18695 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18837 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18730 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18680 {
            return Some(NetherBrickWall {
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 18707 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 18645 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18701 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18844 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 18582 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18877 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 18592 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18658 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 18635 {
            return Some(NetherBrickWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::None,
            });
        }
        if state_id == 18832 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 18598 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18775 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 18847 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 18876 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
            });
        }
        if state_id == 18608 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18805 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18827 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18585 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 18816 {
            return Some(NetherBrickWall {
                r#up: true,
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18574 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 18610 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18579 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18840 {
            return Some(NetherBrickWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18709 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18829 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18779 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18878 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18696 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 18825 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18843 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
            });
        }
        if state_id == 18615 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 18838 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 18850 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18761 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 18588 {
            return Some(NetherBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
            });
        }
        if state_id == 18614 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::None,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 18715 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18583 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18831 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 18728 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18622 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 18839 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 18720 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18601 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 18566 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18657 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18668 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
            });
        }
        if state_id == 18718 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#waterlogged: true,
                r#up: true,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18786 {
            return Some(NetherBrickWall {
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18803 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18660 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 18819 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18875 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 18882 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
            });
        }
        if state_id == 18633 {
            return Some(NetherBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 18642 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 18708 {
            return Some(NetherBrickWall {
                r#up: true,
                r#west: West::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::Low,
            });
        }
        if state_id == 18811 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18806 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 18712 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 18576 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 18704 {
            return Some(NetherBrickWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18603 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#south: South::None,
            });
        }
        if state_id == 18639 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18759 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18666 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#up: false,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 18818 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 18871 {
            return Some(NetherBrickWall {
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18789 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18641 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 18787 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18744 {
            return Some(NetherBrickWall {
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 18747 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 18750 {
            return Some(NetherBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 18676 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 18754 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 18864 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 18742 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18824 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18830 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 18849 {
            return Some(NetherBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 18681 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18643 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18822 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18690 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 18861 {
            return Some(NetherBrickWall {
                r#up: true,
                r#west: West::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 18733 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18813 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 18778 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#up: true,
            });
        }
        if state_id == 18698 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#north: North::None,
            });
        }
        if state_id == 18722 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 18749 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#up: false,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18790 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18688 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::Low,
            });
        }
        if state_id == 18727 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#up: false,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18606 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 18770 {
            return Some(NetherBrickWall {
                r#up: false,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 18763 {
            return Some(NetherBrickWall {
                r#up: false,
                r#north: North::Tall,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18654 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 18572 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::None,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18575 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 18713 {
            return Some(NetherBrickWall {
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 18700 {
            return Some(NetherBrickWall {
                r#up: false,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18647 {
            return Some(NetherBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
                r#west: West::None,
            });
        }
        if state_id == 18800 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 18766 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18804 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 18571 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#south: South::None,
                r#waterlogged: false,
            });
        }
        if state_id == 18578 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::None,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 18667 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 18661 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 18723 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 18785 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 18802 {
            return Some(NetherBrickWall {
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 18865 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18874 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 18841 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 18736 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18737 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 18663 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 18597 {
            return Some(NetherBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 18836 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18872 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18651 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18671 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#up: true,
                r#waterlogged: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 18756 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 18814 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18570 {
            return Some(NetherBrickWall {
                r#west: West::Low,
                r#south: South::None,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 18617 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 18625 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
            });
        }
        if state_id == 18644 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 18687 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18780 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 18863 {
            return Some(NetherBrickWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 18560 {
            return Some(NetherBrickWall {
                r#up: true,
                r#south: South::None,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
            });
        }
        if state_id == 18564 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 18724 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 18765 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18791 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 18705 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 18734 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::Low,
            });
        }
        if state_id == 18809 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18755 {
            return Some(NetherBrickWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18760 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 18764 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: true,
            });
        }
        if state_id == 18729 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 18793 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18679 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#north: North::None,
                r#up: false,
                r#south: South::None,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 18781 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 18852 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 18562 {
            return Some(NetherBrickWall {
                r#north: North::None,
                r#west: West::Tall,
                r#waterlogged: true,
                r#up: true,
                r#east: East::None,
                r#south: South::None,
            });
        }
        if state_id == 18662 {
            return Some(NetherBrickWall {
                r#up: false,
                r#east: East::None,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 18638 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#up: false,
                r#west: West::None,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 18858 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 18650 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 18710 {
            return Some(NetherBrickWall {
                r#up: false,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 18693 {
            return Some(NetherBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Low,
            });
        }
        if state_id == 18782 {
            return Some(NetherBrickWall {
                r#west: West::None,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 18591 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 18788 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 18616 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
            });
        }
        if state_id == 18563 {
            return Some(NetherBrickWall {
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 18703 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18751 {
            return Some(NetherBrickWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 18673 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18646 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18607 {
            return Some(NetherBrickWall {
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 18689 {
            return Some(NetherBrickWall {
                r#up: false,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 18768 {
            return Some(NetherBrickWall {
                r#east: East::Low,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 18773 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Tall,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 18611 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Low,
                r#up: true,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 18619 {
            return Some(NetherBrickWall {
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 18589 {
            return Some(NetherBrickWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
            });
        }
        if state_id == 18621 {
            return Some(NetherBrickWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 18767 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
            });
        }
        if state_id == 18795 {
            return Some(NetherBrickWall {
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 18587 {
            return Some(NetherBrickWall {
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
            });
        }
        if state_id == 18702 {
            return Some(NetherBrickWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 18833 {
            return Some(NetherBrickWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::None,
            });
        }
        if state_id == 18862 {
            return Some(NetherBrickWall {
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 18777 {
            return Some(NetherBrickWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        return None;
    }
}

