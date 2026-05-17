use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackstoneWall {
    pub r#south: South,
    pub waterlogged: bool,
    pub up: bool,
    pub r#north: North,
    pub r#west: West,
    pub r#east: East,
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

impl BlockState for BlackstoneWall {
    fn to_id(&self) -> i32 {
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::None && self.r#east == East::Tall { return 22009; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low { return 21930; }
        if self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::None && self.r#north == North::Tall { return 21788; }
        if self.r#up == true && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Low { return 21822; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Tall { return 22029; }
        if self.r#west == West::None && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == true { return 21773; }
        if self.r#east == East::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Tall { return 21928; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::None { return 21986; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::None && self.r#west == West::Low && self.r#south == South::Tall && self.r#up == false { return 21741; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#west == West::None && self.r#up == false && self.r#east == East::None && self.r#waterlogged == false { return 21755; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Low { return 21904; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#waterlogged == false { return 21991; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == true && self.r#north == North::Tall && self.r#west == West::Low { return 22026; }
        if self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#north == North::None { return 21943; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Tall && self.r#up == true && self.r#west == West::None && self.r#waterlogged == true { return 21914; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 21778; }
        if self.r#up == false && self.r#south == South::Low && self.r#east == East::Low && self.r#west == West::Tall && self.r#waterlogged == false && self.r#north == North::Tall { return 21913; }
        if self.r#south == South::None && self.r#up == false && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#east == East::Low { return 21828; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::None && self.r#east == East::None && self.r#north == North::None { return 21715; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None && self.r#north == North::None && self.r#south == South::None && self.r#up == false { return 21719; }
        if self.r#up == false && self.r#west == West::None && self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall { return 21959; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true && self.r#east == East::Tall && self.r#south == South::None { return 21964; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None && self.r#up == false { return 21973; }
        if self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::Low && self.r#up == true && self.r#west == West::Tall && self.r#east == East::None { return 21772; }
        if self.r#east == East::Low && self.r#up == false && self.r#west == West::Tall && self.r#waterlogged == true && self.r#north == North::Low && self.r#south == South::Tall { return 21886; }
        if self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#east == East::Tall { return 21949; }
        if self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::Tall { return 21951; }
        if self.r#west == West::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false { return 21763; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None && self.r#up == true { return 21926; }
        if self.r#west == West::Low && self.r#up == false && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == true { return 21849; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#up == false && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == true { return 21718; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::None && self.r#east == East::Tall { return 21960; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#up == false && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::Low { return 21994; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#east == East::Low { return 21836; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Low && self.r#up == true { return 21726; }
        if self.r#west == West::Tall && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::Tall { return 22024; }
        if self.r#south == South::Low && self.r#up == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 21910; }
        if self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::None && self.r#up == false && self.r#east == East::Low && self.r#north == North::Tall { return 21896; }
        if self.r#waterlogged == true && self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::Tall { return 21915; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#up == false && self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::Low { return 21789; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::None && self.r#up == false && self.r#south == South::Low && self.r#waterlogged == true { return 21945; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#up == false && self.r#south == South::Tall && self.r#west == West::Tall && self.r#waterlogged == false { return 21853; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None { return 21749; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::None && self.r#west == West::None && self.r#south == South::Tall && self.r#up == false { return 21740; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Low && self.r#up == true && self.r#south == South::None && self.r#west == West::Tall { return 21856; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Low && self.r#up == true { return 21882; }
        if self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true { return 21891; }
        if self.r#up == true && self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::None { return 21713; }
        if self.r#east == East::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#up == false && self.r#north == North::Low && self.r#west == West::Tall { return 21997; }
        if self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::None && self.r#up == true && self.r#west == West::None { return 21821; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == false && self.r#up == false && self.r#south == South::None && self.r#west == West::Tall { return 21865; }
        if self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::Low && self.r#up == true { return 21866; }
        if self.r#west == West::Tall && self.r#up == false && self.r#south == South::Low && self.r#east == East::Tall && self.r#north == North::Tall && self.r#waterlogged == false { return 22021; }
        if self.r#south == South::None && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::None && self.r#up == true && self.r#north == North::Low { return 21857; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::None && self.r#up == true && self.r#east == East::Tall && self.r#north == North::None { return 21929; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Tall { return 21814; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::None { return 21850; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Low { return 21807; }
        if self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#south == South::Tall { return 21916; }
        if self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::Low { return 21922; }
        if self.r#waterlogged == true && self.r#west == West::None && self.r#up == false && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Tall { return 22016; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false { return 21816; }
        if self.r#waterlogged == false && self.r#up == false && self.r#north == North::None && self.r#south == South::Tall && self.r#west == West::None && self.r#east == East::Low { return 21851; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::None { return 21980; }
        if self.r#up == false && self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::Tall && self.r#west == West::Tall && self.r#south == South::Tall { return 21958; }
        if self.r#west == West::None && self.r#south == South::None && self.r#north == North::None && self.r#waterlogged == true && self.r#up == false && self.r#east == East::Low { return 21824; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false { return 22014; }
        if self.r#waterlogged == true && self.r#up == false && self.r#east == East::Tall && self.r#west == West::Tall && self.r#south == South::Tall && self.r#north == North::Tall { return 22030; }
        if self.r#west == West::None && self.r#east == East::None && self.r#waterlogged == false && self.r#up == false && self.r#north == North::Tall && self.r#south == South::Tall { return 21815; }
        if self.r#east == East::None && self.r#south == South::Tall && self.r#west == West::Tall && self.r#north == North::None && self.r#up == false && self.r#waterlogged == false { return 21745; }
        if self.r#up == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#south == South::None { return 21899; }
        if self.r#waterlogged == true && self.r#up == true && self.r#west == West::Low && self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::Tall { return 21987; }
        if self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low { return 22011; }
        if self.r#up == false && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::Low && self.r#west == West::None && self.r#waterlogged == true { return 21764; }
        if self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::None && self.r#waterlogged == true && self.r#north == North::None && self.r#up == false { return 21933; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall { return 22015; }
        if self.r#up == false && self.r#west == West::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::None { return 21848; }
        if self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::Low && self.r#up == false { return 21888; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall { return 21988; }
        if self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Tall { return 21811; }
        if self.r#west == West::None && self.r#waterlogged == false && self.r#up == true && self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Tall { return 22025; }
        if self.r#north == North::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#south == South::Low { return 21732; }
        if self.r#waterlogged == false && self.r#east == East::None && self.r#up == true && self.r#north == North::Tall && self.r#west == West::None && self.r#south == South::Tall { return 21809; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true && self.r#south == South::None { return 22004; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#west == West::None { return 21779; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::None { return 21767; }
        if self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::Tall { return 21817; }
        if self.r#waterlogged == false && self.r#up == false && self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::Tall { return 21925; }
        if self.r#south == South::None && self.r#west == West::Low && self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::None { return 21711; }
        if self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low { return 21877; }
        if self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Tall && self.r#north == North::Tall && self.r#south == South::None && self.r#up == false { return 21898; }
        if self.r#north == North::None && self.r#south == South::Tall && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall { return 21952; }
        if self.r#east == East::Low && self.r#up == true && self.r#west == West::None && self.r#north == North::None && self.r#south == South::Low && self.r#waterlogged == false { return 21833; }
        if self.r#east == East::Tall && self.r#up == false && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::Tall { return 21992; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#south == South::Tall && self.r#up == true && self.r#east == East::None && self.r#waterlogged == true { return 21770; }
        if self.r#south == South::Low && self.r#east == East::Low && self.r#up == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#waterlogged == true { return 21874; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low && self.r#south == South::None && self.r#up == false && self.r#north == North::Tall { return 21897; }
        if self.r#up == true && self.r#west == West::None && self.r#waterlogged == true && self.r#north == North::None && self.r#east == East::None && self.r#south == South::Low { return 21722; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Low { return 21894; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == true && self.r#west == West::Low && self.r#south == South::None { return 21966; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Low && self.r#up == false && self.r#east == East::None { return 21792; }
        if self.r#up == true && self.r#west == West::None && self.r#south == South::Tall && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Tall { return 21950; }
        if self.r#waterlogged == true && self.r#up == true && self.r#west == West::Low && self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Tall { return 21939; }
        if self.r#up == true && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Low { return 21799; }
        if self.r#west == West::None && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low && self.r#south == South::Low && self.r#north == North::Low { return 21875; }
        if self.r#north == North::None && self.r#south == South::None && self.r#east == East::Low && self.r#waterlogged == false && self.r#west == West::Tall && self.r#up == true { return 21823; }
        if self.r#west == West::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#up == true && self.r#north == North::None { return 21830; }
        if self.r#waterlogged == true && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == true && self.r#south == South::Low && self.r#west == West::None { return 21974; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#up == true && self.r#west == West::None && self.r#north == North::Low && self.r#waterlogged == false { return 21761; }
        if self.r#up == false && self.r#east == East::Low && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall { return 21924; }
        if self.r#north == North::None && self.r#west == West::None && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall { return 21935; }
        if self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::None && self.r#waterlogged == false && self.r#up == true && self.r#east == East::Tall { return 21931; }
        if self.r#west == West::None && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall { return 21995; }
        if self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::Tall && self.r#south == South::None && self.r#north == North::Tall { return 22002; }
        if self.r#south == South::Low && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == false { return 21985; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Tall && self.r#up == false && self.r#north == North::Tall && self.r#south == South::None { return 22007; }
        if self.r#north == North::Tall && self.r#south == South::Tall && self.r#west == West::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == true { return 22027; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#up == false && self.r#south == South::Tall && self.r#west == West::None && self.r#waterlogged == true { return 22028; }
        if self.r#west == West::None && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Tall && self.r#north == North::Tall { return 21812; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::None && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None { return 21782; }
        if self.r#up == true && self.r#north == North::Tall && self.r#east == East::Tall && self.r#west == West::Tall && self.r#waterlogged == true && self.r#south == South::Low { return 22012; }
        if self.r#up == false && self.r#south == South::None && self.r#north == North::None && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Tall { return 21937; }
        if self.r#east == East::None && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::None { return 21756; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#west == West::Low && self.r#waterlogged == false && self.r#south == South::None { return 21858; }
        if self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == true && self.r#north == North::None { return 21832; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#north == North::Tall { return 22032; }
        if self.r#up == true && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::None && self.r#north == North::Tall && self.r#east == East::None { return 21797; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::None { return 21786; }
        if self.r#east == East::None && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::Low { return 21771; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#east == East::None && self.r#up == true && self.r#south == South::Tall { return 21737; }
        if self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::None && self.r#east == East::Low && self.r#south == South::Tall { return 21884; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#north == North::Low && self.r#waterlogged == false && self.r#up == false && self.r#west == West::None { return 21863; }
        if self.r#up == false && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#south == South::None && self.r#east == East::Tall { return 22005; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#east == East::Tall { return 21967; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#up == false { return 21729; }
        if self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::None && self.r#waterlogged == false && self.r#north == North::Tall && self.r#up == true { return 21787; }
        if self.r#south == South::Tall && self.r#up == true && self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#west == West::Low { return 21879; }
        if self.r#east == East::None && self.r#north == North::None && self.r#west == West::Tall && self.r#up == true && self.r#south == South::Tall && self.r#waterlogged == true { return 21736; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::None && self.r#up == false && self.r#west == West::Tall { return 21754; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#south == South::None && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == true { return 21969; }
        if self.r#east == East::Tall && self.r#up == false && self.r#west == West::Tall && self.r#south == South::Low && self.r#waterlogged == true && self.r#north == North::Tall { return 22018; }
        if self.r#south == South::Low && self.r#north == North::Tall && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#east == East::Tall { return 22019; }
        if self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::None && self.r#west == West::Tall { return 21781; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::Low && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == false { return 21780; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false { return 22033; }
        if self.r#south == South::Low && self.r#north == North::None && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None { return 21731; }
        if self.r#up == false && self.r#north == North::None && self.r#waterlogged == true && self.r#east == East::Tall && self.r#west == West::None && self.r#south == South::Tall { return 21956; }
        if self.r#east == East::Low && self.r#up == true && self.r#west == West::Low && self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::Tall { return 21906; }
        if self.r#waterlogged == true && self.r#up == false && self.r#south == South::None && self.r#west == West::None && self.r#east == East::Tall && self.r#north == North::Low { return 21968; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == true { return 21908; }
        if self.r#up == false && self.r#south == South::None && self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::None { return 21932; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#east == East::Tall && self.r#west == West::Low && self.r#up == false && self.r#north == North::None { return 21957; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#east == East::Low && self.r#west == West::Tall && self.r#south == South::Low && self.r#up == true { return 21907; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#south == South::Low && self.r#east == East::Low && self.r#up == true && self.r#west == West::Tall { return 21871; }
        if self.r#south == South::None && self.r#up == true && self.r#north == North::Low && self.r#west == West::None && self.r#east == East::Low && self.r#waterlogged == true { return 21854; }
        if self.r#east == East::None && self.r#up == false && self.r#north == North::None && self.r#west == West::Tall && self.r#south == South::Tall && self.r#waterlogged == true { return 21742; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#south == South::Tall && self.r#west == West::Tall && self.r#waterlogged == false { return 21883; }
        if self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Tall { return 21806; }
        if self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 21720; }
        if self.r#up == false && self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#west == West::None && self.r#waterlogged == true { return 21716; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::Tall && self.r#south == South::Tall && self.r#up == false && self.r#waterlogged == false { return 21889; }
        if self.r#up == true && self.r#waterlogged == false && self.r#south == South::Low && self.r#north == North::None && self.r#east == East::Low && self.r#west == West::Tall { return 21835; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Tall && self.r#up == false { return 21769; }
        if self.r#south == South::None && self.r#up == false && self.r#east == East::None && self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::None { return 21791; }
        if self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Low { return 21885; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::None && self.r#up == false { return 21934; }
        if self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::Low && self.r#up == true && self.r#south == South::Low { return 21942; }
        if self.r#north == North::Tall && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::Low && self.r#up == false && self.r#west == West::Low { return 22017; }
        if self.r#south == South::None && self.r#up == false && self.r#north == North::None && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Tall { return 21721; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#east == East::Low && self.r#up == true { return 21845; }
        if self.r#waterlogged == true && self.r#up == false && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Tall { return 21920; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Tall && self.r#south == South::None && self.r#north == North::Tall && self.r#up == true { return 22000; }
        if self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::Low && self.r#east == East::None { return 21775; }
        if self.r#east == East::Low && self.r#south == South::None && self.r#north == North::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 21864; }
        if self.r#waterlogged == false && self.r#east == East::Tall && self.r#up == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#west == West::None { return 22013; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#waterlogged == false && self.r#up == false && self.r#west == West::Low && self.r#south == South::Tall { return 21852; }
        if self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Low && self.r#south == South::Low { return 21768; }
        if self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true { return 21796; }
        if self.r#south == South::None && self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Tall { return 21751; }
        if self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::None && self.r#west == West::Low && self.r#up == false { return 21753; }
        if self.r#up == false && self.r#waterlogged == true && self.r#east == East::None && self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::Tall { return 21801; }
        if self.r#south == South::Tall && self.r#east == East::None && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low { return 21774; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#up == false && self.r#south == South::None && self.r#west == West::Tall && self.r#north == North::Tall { return 21790; }
        if self.r#west == West::Tall && self.r#south == South::Tall && self.r#up == true && self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::Tall { return 21919; }
        if self.r#up == true && self.r#east == East::None && self.r#south == South::Low && self.r#north == North::Low && self.r#west == West::None && self.r#waterlogged == true { return 21758; }
        if self.r#up == false && self.r#waterlogged == false && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#west == West::None { return 21983; }
        if self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::None && self.r#west == West::None && self.r#north == North::None && self.r#up == false { return 21743; }
        if self.r#east == East::Tall && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == true && self.r#up == true && self.r#west == West::None { return 22022; }
        if self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Low && self.r#north == North::Low && self.r#west == West::Low { return 21861; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#up == false && self.r#south == South::Low && self.r#west == West::Low { return 21765; }
        if self.r#west == West::Low && self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false { return 21948; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::None { return 21963; }
        if self.r#up == false && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Low && self.r#west == West::Tall && self.r#east == East::Tall { return 21982; }
        if self.r#up == true && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::Low && self.r#waterlogged == false && self.r#north == North::None { return 21834; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#west == West::None && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == true { return 22010; }
        if self.r#west == West::Low && self.r#waterlogged == true && self.r#south == South::None && self.r#east == East::None && self.r#north == North::Low && self.r#up == true { return 21747; }
        if self.r#north == North::Low && self.r#up == false && self.r#south == South::Low && self.r#east == East::Tall && self.r#waterlogged == false && self.r#west == West::Low { return 21984; }
        if self.r#up == false && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Low { return 21923; }
        if self.r#east == East::Low && self.r#south == South::Tall && self.r#up == true && self.r#west == West::None && self.r#north == North::Low && self.r#waterlogged == true { return 21878; }
        if self.r#up == true && self.r#west == West::Tall && self.r#north == North::None && self.r#south == South::Tall && self.r#east == East::Low && self.r#waterlogged == false { return 21847; }
        if self.r#up == true && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::None && self.r#east == East::None && self.r#north == North::None { return 21725; }
        if self.r#south == South::None && self.r#waterlogged == true && self.r#up == false && self.r#east == East::None && self.r#west == West::None && self.r#north == North::Low { return 21752; }
        if self.r#up == true && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == false { return 21881; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Low && self.r#up == false { return 21839; }
        if self.r#up == true && self.r#south == South::Low && self.r#north == North::Tall && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low { return 21903; }
        if self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Low { return 21868; }
        if self.r#east == East::None && self.r#north == North::None && self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 21730; }
        if self.r#waterlogged == false && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::None && self.r#south == South::Low && self.r#up == false { return 21733; }
        if self.r#west == West::Tall && self.r#north == North::Low && self.r#waterlogged == true && self.r#east == East::Tall && self.r#south == South::None && self.r#up == false { return 21970; }
        if self.r#waterlogged == true && self.r#east == East::None && self.r#south == South::None && self.r#north == North::None && self.r#up == true && self.r#west == West::None { return 21710; }
        if self.r#up == true && self.r#west == West::Tall && self.r#south == South::None && self.r#north == North::Tall && self.r#waterlogged == false && self.r#east == East::Low { return 21895; }
        if self.r#east == East::Tall && self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false { return 21978; }
        if self.r#north == North::Low && self.r#east == East::None && self.r#west == West::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false { return 21757; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::None && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low { return 21805; }
        if self.r#north == North::Tall && self.r#south == South::Low && self.r#waterlogged == true && self.r#west == West::None && self.r#up == false && self.r#east == East::None { return 21800; }
        if self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#up == false && self.r#west == West::Low { return 21837; }
        if self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#south == South::Tall && self.r#waterlogged == false && self.r#east == East::Low { return 21918; }
        if self.r#waterlogged == false && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::None && self.r#up == false && self.r#west == West::Tall { return 21793; }
        if self.r#south == South::Low && self.r#up == true && self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == true && self.r#east == East::Tall { return 21938; }
        if self.r#east == East::Tall && self.r#west == West::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low { return 22020; }
        if self.r#west == West::Low && self.r#south == South::Low && self.r#north == North::Tall && self.r#east == East::None && self.r#waterlogged == true && self.r#up == true { return 21795; }
        if self.r#up == true && self.r#east == East::None && self.r#north == North::None && self.r#west == West::None && self.r#waterlogged == true && self.r#south == South::Tall { return 21734; }
        if self.r#south == South::Tall && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None { return 21810; }
        if self.r#south == South::None && self.r#up == true && self.r#east == East::None && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Tall { return 21712; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::None { return 21829; }
        if self.r#north == North::None && self.r#west == West::Tall && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Low && self.r#south == South::None { return 21826; }
        if self.r#up == true && self.r#north == North::Low && self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Tall && self.r#south == South::Low { return 21760; }
        if self.r#north == North::Low && self.r#west == West::None && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Low { return 21860; }
        if self.r#north == North::Low && self.r#west == West::None && self.r#up == false && self.r#waterlogged == true && self.r#south == South::Low && self.r#east == East::Low { return 21872; }
        if self.r#north == North::Low && self.r#west == West::Tall && self.r#up == true && self.r#east == East::Low && self.r#south == South::Tall && self.r#waterlogged == true { return 21880; }
        if self.r#east == East::Tall && self.r#west == West::None && self.r#waterlogged == false && self.r#up == true && self.r#south == South::Low && self.r#north == North::Low { return 21977; }
        if self.r#up == true && self.r#east == East::None && self.r#north == North::None && self.r#south == South::None && self.r#waterlogged == false && self.r#west == West::Low { return 21714; }
        if self.r#up == false && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::None { return 21717; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::None && self.r#south == South::None { return 21818; }
        if self.r#north == North::Low && self.r#south == South::Tall && self.r#east == East::Tall && self.r#waterlogged == false && self.r#up == true && self.r#west == West::Low { return 21990; }
        if self.r#north == North::None && self.r#east == East::Tall && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Tall { return 21954; }
        if self.r#waterlogged == true && self.r#north == North::Tall && self.r#east == East::None && self.r#up == false && self.r#south == South::Low && self.r#west == West::Tall { return 21802; }
        if self.r#east == East::None && self.r#west == West::Low && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Tall && self.r#up == false { return 21744; }
        if self.r#up == true && self.r#waterlogged == false && self.r#east == East::Tall && self.r#south == South::Tall && self.r#west == West::None && self.r#north == North::Low { return 21989; }
        if self.r#east == East::None && self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == false { return 21798; }
        if self.r#west == West::Tall && self.r#north == North::None && self.r#waterlogged == false && self.r#up == true && self.r#east == East::Tall && self.r#south == South::Tall { return 21955; }
        if self.r#north == North::None && self.r#east == East::Low && self.r#south == South::Low && self.r#west == West::Low && self.r#up == false && self.r#waterlogged == false { return 21840; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::Tall && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Low { return 21946; }
        if self.r#up == false && self.r#north == North::None && self.r#waterlogged == false && self.r#south == South::Tall && self.r#east == East::Tall && self.r#west == West::Tall { return 21961; }
        if self.r#up == true && self.r#east == East::None && self.r#south == South::None && self.r#west == West::None && self.r#north == North::Low && self.r#waterlogged == true { return 21746; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low { return 21855; }
        if self.r#waterlogged == true && self.r#up == true && self.r#south == South::None && self.r#east == East::Low && self.r#north == North::None && self.r#west == West::Tall { return 21820; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::None && self.r#up == true && self.r#south == South::Tall && self.r#east == East::Tall { return 21953; }
        if self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall && self.r#south == South::Low && self.r#east == East::None { return 21803; }
        if self.r#west == West::None && self.r#south == South::Low && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::None && self.r#up == false { return 21728; }
        if self.r#south == South::None && self.r#east == East::None && self.r#waterlogged == true && self.r#up == true && self.r#west == West::Tall && self.r#north == North::Low { return 21748; }
        if self.r#south == South::Low && self.r#up == true && self.r#north == North::None && self.r#waterlogged == false && self.r#east == East::Tall && self.r#west == West::None { return 21941; }
        if self.r#up == true && self.r#west == West::None && self.r#south == South::None && self.r#north == North::Low && self.r#east == East::Tall && self.r#waterlogged == true { return 21962; }
        if self.r#east == East::Tall && self.r#up == false && self.r#south == South::Tall && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall { return 22031; }
        if self.r#waterlogged == false && self.r#north == North::None && self.r#west == West::Tall && self.r#east == East::None && self.r#south == South::Low && self.r#up == true { return 21727; }
        if self.r#waterlogged == true && self.r#south == South::None && self.r#up == true && self.r#east == East::None && self.r#north == North::Tall && self.r#west == West::Low { return 21783; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#up == true && self.r#east == East::Tall && self.r#waterlogged == true && self.r#west == West::None { return 21998; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#south == South::Low && self.r#east == East::None && self.r#up == true && self.r#north == North::Low { return 21762; }
        if self.r#north == North::None && self.r#south == South::None && self.r#east == East::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None { return 21827; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::Low && self.r#up == true { return 21867; }
        if self.r#west == West::Tall && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::None && self.r#up == true && self.r#waterlogged == true { return 21892; }
        if self.r#east == East::Low && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Low && self.r#north == North::None { return 21831; }
        if self.r#waterlogged == true && self.r#north == North::Low && self.r#up == false && self.r#east == East::Tall && self.r#south == South::Low && self.r#west == West::Low { return 21981; }
        if self.r#south == South::Low && self.r#up == false && self.r#waterlogged == true && self.r#east == East::Low && self.r#west == West::Tall && self.r#north == North::None { return 21838; }
        if self.r#north == North::Tall && self.r#waterlogged == false && self.r#west == West::Low && self.r#up == false && self.r#east == East::Tall && self.r#south == South::None { return 22008; }
        if self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::None && self.r#up == true && self.r#south == South::Low && self.r#north == North::None { return 21723; }
        if self.r#up == false && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Tall { return 21912; }
        if self.r#south == South::Tall && self.r#east == East::Tall && self.r#up == true && self.r#west == West::Low && self.r#north == North::Tall && self.r#waterlogged == true { return 22023; }
        if self.r#east == East::None && self.r#up == true && self.r#waterlogged == false && self.r#north == North::None && self.r#south == South::Tall && self.r#west == West::Tall { return 21739; }
        if self.r#north == North::Tall && self.r#up == true && self.r#east == East::Tall && self.r#south == South::None && self.r#west == West::None && self.r#waterlogged == false { return 22001; }
        if self.r#waterlogged == true && self.r#north == North::None && self.r#south == South::None && self.r#east == East::Low && self.r#up == true && self.r#west == West::Low { return 21819; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#up == true && self.r#south == South::Low && self.r#west == West::None && self.r#waterlogged == false { return 21905; }
        if self.r#north == North::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == true && self.r#east == East::Low && self.r#south == South::Tall { return 21843; }
        if self.r#south == South::Tall && self.r#north == North::Tall && self.r#west == West::Low && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true { return 21813; }
        if self.r#east == East::None && self.r#waterlogged == true && self.r#west == West::Low && self.r#up == false && self.r#north == North::Low && self.r#south == South::Tall { return 21777; }
        if self.r#west == West::Low && self.r#north == North::Tall && self.r#up == false && self.r#waterlogged == false && self.r#south == South::None && self.r#east == East::Low { return 21900; }
        if self.r#south == South::None && self.r#east == East::Tall && self.r#up == false && self.r#west == West::None && self.r#waterlogged == false && self.r#north == North::Low { return 21971; }
        if self.r#north == North::None && self.r#up == false && self.r#west == West::Tall && self.r#east == East::Low && self.r#waterlogged == false && self.r#south == South::Low { return 21841; }
        if self.r#east == East::Low && self.r#north == North::Tall && self.r#up == true && self.r#waterlogged == true && self.r#south == South::None && self.r#west == West::None { return 21890; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#south == South::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::None { return 21869; }
        if self.r#west == West::Low && self.r#up == true && self.r#east == East::None && self.r#waterlogged == true && self.r#south == South::Low && self.r#north == North::Low { return 21759; }
        if self.r#east == East::Tall && self.r#waterlogged == false && self.r#south == South::None && self.r#west == West::Tall && self.r#north == North::Tall && self.r#up == true { return 22003; }
        if self.r#south == South::Low && self.r#up == false && self.r#waterlogged == false && self.r#west == West::None && self.r#east == East::Low && self.r#north == North::Tall { return 21911; }
        if self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 21936; }
        if self.r#waterlogged == true && self.r#up == false && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Tall && self.r#south == South::Low { return 21909; }
        if self.r#west == West::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#up == true && self.r#east == East::None && self.r#waterlogged == true { return 21735; }
        if self.r#south == South::Tall && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None { return 21846; }
        if self.r#waterlogged == false && self.r#west == West::None && self.r#south == South::Tall && self.r#up == false && self.r#north == North::Low && self.r#east == East::Low { return 21887; }
        if self.r#north == North::Tall && self.r#up == true && self.r#south == South::Low && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::None { return 21902; }
        if self.r#west == West::Tall && self.r#east == East::Tall && self.r#north == North::Low && self.r#up == true && self.r#south == South::Low && self.r#waterlogged == true { return 21976; }
        if self.r#waterlogged == true && self.r#south == South::Tall && self.r#up == false && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::Tall { return 21921; }
        if self.r#south == South::Low && self.r#up == true && self.r#waterlogged == true && self.r#east == East::None && self.r#north == North::Tall && self.r#west == West::None { return 21794; }
        if self.r#west == West::Low && self.r#south == South::None && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#north == North::Low { return 21972; }
        if self.r#up == true && self.r#waterlogged == true && self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::Low && self.r#east == East::Tall { return 21975; }
        if self.r#waterlogged == false && self.r#west == West::Low && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::None && self.r#up == true { return 21750; }
        if self.r#east == East::Low && self.r#north == North::Low && self.r#up == true && self.r#waterlogged == false && self.r#west == West::Tall && self.r#south == South::None { return 21859; }
        if self.r#north == North::Low && self.r#south == South::None && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == false { return 21862; }
        if self.r#up == false && self.r#north == North::Low && self.r#west == West::Low && self.r#south == South::Tall && self.r#waterlogged == true && self.r#east == East::Tall { return 21993; }
        if self.r#east == East::Tall && self.r#waterlogged == true && self.r#south == South::None && self.r#north == North::None && self.r#west == West::Low && self.r#up == true { return 21927; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::None && self.r#north == North::Tall && self.r#up == true && self.r#south == South::None { return 21785; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#west == West::None && self.r#east == East::Low && self.r#up == true && self.r#waterlogged == false { return 21893; }
        if self.r#west == West::None && self.r#north == North::None && self.r#south == South::Low && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == true { return 21944; }
        if self.r#up == false && self.r#waterlogged == true && self.r#west == West::Low && self.r#east == East::Low && self.r#north == North::None && self.r#south == South::None { return 21825; }
        if self.r#waterlogged == false && self.r#north == North::Low && self.r#west == West::Low && self.r#east == East::Low && self.r#up == false && self.r#south == South::Low { return 21876; }
        if self.r#south == South::None && self.r#north == North::Tall && self.r#up == true && self.r#west == West::Low && self.r#east == East::Tall && self.r#waterlogged == true { return 21999; }
        if self.r#east == East::None && self.r#north == North::Tall && self.r#up == true && self.r#south == South::None && self.r#west == West::Tall && self.r#waterlogged == true { return 21784; }
        if self.r#south == South::Tall && self.r#up == true && self.r#east == East::None && self.r#waterlogged == false && self.r#west == West::Low && self.r#north == North::None { return 21738; }
        if self.r#east == East::None && self.r#waterlogged == false && self.r#south == South::Low && self.r#west == West::Low && self.r#north == North::Tall && self.r#up == false { return 21804; }
        if self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall && self.r#north == North::None && self.r#east == East::None && self.r#south == South::Low { return 21724; }
        if self.r#south == South::Low && self.r#north == North::Low && self.r#east == East::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 21766; }
        if self.r#west == West::None && self.r#up == false && self.r#east == East::None && self.r#north == North::Low && self.r#south == South::Tall && self.r#waterlogged == true { return 21776; }
        if self.r#up == true && self.r#waterlogged == true && self.r#north == North::None && self.r#west == West::None && self.r#south == South::Tall && self.r#east == East::Low { return 21842; }
        if self.r#east == East::Low && self.r#north == North::None && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true && self.r#west == West::Tall { return 21844; }
        if self.r#north == North::Low && self.r#south == South::Low && self.r#up == false && self.r#east == East::Low && self.r#waterlogged == true && self.r#west == West::Low { return 21873; }
        if self.r#west == West::Tall && self.r#north == North::Tall && self.r#east == East::None && self.r#south == South::Tall && self.r#up == true && self.r#waterlogged == true { return 21808; }
        if self.r#west == West::Low && self.r#waterlogged == false && self.r#south == South::Low && self.r#north == North::Low && self.r#up == true && self.r#east == East::Low { return 21870; }
        if self.r#north == North::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == false && self.r#east == East::Low && self.r#west == West::Tall { return 21901; }
        if self.r#waterlogged == true && self.r#west == West::Tall && self.r#up == true && self.r#east == East::Tall && self.r#north == North::None && self.r#south == South::Low { return 21940; }
        if self.r#south == South::Low && self.r#west == West::None && self.r#north == North::None && self.r#up == false && self.r#east == East::Tall && self.r#waterlogged == false { return 21947; }
        if self.r#up == true && self.r#west == West::None && self.r#north == North::Low && self.r#south == South::None && self.r#east == East::Tall && self.r#waterlogged == false { return 21965; }
        if self.r#east == East::Tall && self.r#north == North::Low && self.r#south == South::Low && self.r#west == West::Tall && self.r#up == true && self.r#waterlogged == false { return 21979; }
        if self.r#south == South::Tall && self.r#north == North::Low && self.r#east == East::Tall && self.r#up == false && self.r#waterlogged == false && self.r#west == West::Low { return 21996; }
        if self.r#north == North::Tall && self.r#east == East::Tall && self.r#south == South::None && self.r#up == false && self.r#waterlogged == true && self.r#west == West::Tall { return 22006; }
        if self.r#waterlogged == false && self.r#up == true && self.r#north == North::Tall && self.r#east == East::Low && self.r#south == South::Tall && self.r#west == West::None { return 21917; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22009 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 21930 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 21788 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::None,
                r#north: North::Tall,
            });
        }
        if state_id == 21822 {
            return Some(BlackstoneWall {
                r#up: true,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Low,
            });
        }
        if state_id == 22029 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 21773 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 21928 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 21986 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 21741 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::Low,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 21755 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#south: South::None,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#waterlogged: false,
            });
        }
        if state_id == 21904 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 21991 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 22026 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 21943 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 21914 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 21778 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21913 {
            return Some(BlackstoneWall {
                r#up: false,
                r#south: South::Low,
                r#east: East::Low,
                r#west: West::Tall,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 21828 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#up: false,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#east: East::Low,
            });
        }
        if state_id == 21715 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 21719 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 21959 {
            return Some(BlackstoneWall {
                r#up: false,
                r#west: West::None,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 21964 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 21973 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 21772 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#up: true,
                r#west: West::Tall,
                r#east: East::None,
            });
        }
        if state_id == 21886 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#waterlogged: true,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21949 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 21951 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21763 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21926 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 21849 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#up: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: true,
            });
        }
        if state_id == 21718 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: true,
            });
        }
        if state_id == 21960 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::None,
                r#east: East::Tall,
            });
        }
        if state_id == 21994 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 21836 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 21726 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 22024 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 21910 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21896 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 21915 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21789 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::Low,
            });
        }
        if state_id == 21945 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 21853 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#up: false,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 21749 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 21740 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 21856 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Low,
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 21882 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 21891 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 21713 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 21997 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 21821 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 21865 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
            });
        }
        if state_id == 21866 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 22021 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#north: North::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 21857 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 21929 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::None,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
            });
        }
        if state_id == 21814 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
            });
        }
        if state_id == 21850 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::None,
            });
        }
        if state_id == 21807 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
            });
        }
        if state_id == 21916 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21922 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 22016 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 21816 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21851 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#up: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::None,
                r#east: East::Low,
            });
        }
        if state_id == 21980 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 21958 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 21824 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::Low,
            });
        }
        if state_id == 22014 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 22030 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#up: false,
                r#east: East::Tall,
                r#west: West::Tall,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 21815 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#east: East::None,
                r#waterlogged: false,
                r#up: false,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 21745 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21899 {
            return Some(BlackstoneWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 21987 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 22011 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 21764 {
            return Some(BlackstoneWall {
                r#up: false,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 21933 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#waterlogged: true,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 22015 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 21848 {
            return Some(BlackstoneWall {
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::None,
            });
        }
        if state_id == 21888 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 21988 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 21811 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 22025 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 21732 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 21809 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#east: East::None,
                r#up: true,
                r#north: North::Tall,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 22004 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
            });
        }
        if state_id == 21779 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        if state_id == 21767 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::None,
            });
        }
        if state_id == 21817 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 21925 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21711 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#west: West::Low,
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
            });
        }
        if state_id == 21877 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 21898 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 21952 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21833 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 21992 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#up: false,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 21770 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 21874 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#east: East::Low,
                r#up: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 21897 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#south: South::None,
                r#up: false,
                r#north: North::Tall,
            });
        }
        if state_id == 21722 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::None,
                r#waterlogged: true,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 21894 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 21966 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 21792 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 21950 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::None,
                r#south: South::Tall,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 21939 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#up: true,
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 21799 {
            return Some(BlackstoneWall {
                r#up: true,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 21875 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 21823 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: false,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 21830 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 21974 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 21761 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 21924 {
            return Some(BlackstoneWall {
                r#up: false,
                r#east: East::Low,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21935 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 21931 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
            });
        }
        if state_id == 21995 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 22002 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
            });
        }
        if state_id == 21985 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: false,
            });
        }
        if state_id == 22007 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Tall,
                r#up: false,
                r#north: North::Tall,
                r#south: South::None,
            });
        }
        if state_id == 22027 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::Tall,
                r#west: West::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
            });
        }
        if state_id == 22028 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 21812 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 21782 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
            });
        }
        if state_id == 22012 {
            return Some(BlackstoneWall {
                r#up: true,
                r#north: North::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
                r#waterlogged: true,
                r#south: South::Low,
            });
        }
        if state_id == 21937 {
            return Some(BlackstoneWall {
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 21756 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 21858 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 21832 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
            });
        }
        if state_id == 22032 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 21797 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#north: North::Tall,
                r#east: East::None,
            });
        }
        if state_id == 21786 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::None,
            });
        }
        if state_id == 21771 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::Low,
            });
        }
        if state_id == 21737 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#east: East::None,
                r#up: true,
                r#south: South::Tall,
            });
        }
        if state_id == 21884 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::None,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21863 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#waterlogged: false,
                r#up: false,
                r#west: West::None,
            });
        }
        if state_id == 22005 {
            return Some(BlackstoneWall {
                r#up: false,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
            });
        }
        if state_id == 21967 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 21729 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
            });
        }
        if state_id == 21787 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::None,
                r#waterlogged: false,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 21879 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 21736 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#north: North::None,
                r#west: West::Tall,
                r#up: true,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 21754 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 21969 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 22018 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#up: false,
                r#west: West::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#north: North::Tall,
            });
        }
        if state_id == 22019 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#north: North::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#east: East::Tall,
            });
        }
        if state_id == 21781 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::None,
                r#west: West::Tall,
            });
        }
        if state_id == 21780 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 22033 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21731 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#north: North::None,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 21956 {
            return Some(BlackstoneWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: true,
                r#east: East::Tall,
                r#west: West::None,
                r#south: South::Tall,
            });
        }
        if state_id == 21906 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 21968 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 21908 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 21932 {
            return Some(BlackstoneWall {
                r#up: false,
                r#south: South::None,
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 21957 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: false,
                r#north: North::None,
            });
        }
        if state_id == 21907 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::Low,
                r#west: West::Tall,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 21871 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#up: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21854 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#up: true,
                r#north: North::Low,
                r#west: West::None,
                r#east: East::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 21742 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#up: false,
                r#north: North::None,
                r#west: West::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 21883 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#south: South::Tall,
                r#west: West::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 21806 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 21720 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 21716 {
            return Some(BlackstoneWall {
                r#up: false,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 21889 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21835 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::None,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 21769 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 21791 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#up: false,
                r#east: East::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 21885 {
            return Some(BlackstoneWall {
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 21934 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 21942 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::Low,
                r#up: true,
                r#south: South::Low,
            });
        }
        if state_id == 22017 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::Low,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 21721 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#up: false,
                r#north: North::None,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Tall,
            });
        }
        if state_id == 21845 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
            });
        }
        if state_id == 21920 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 22000 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 21775 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::Low,
                r#east: East::None,
            });
        }
        if state_id == 21864 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#south: South::None,
                r#north: North::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 22013 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#east: East::Tall,
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 21852 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#waterlogged: false,
                r#up: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21768 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Low,
            });
        }
        if state_id == 21796 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
            });
        }
        if state_id == 21751 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 21753 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#west: West::Low,
                r#up: false,
            });
        }
        if state_id == 21801 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#east: East::None,
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 21774 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 21790 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Tall,
            });
        }
        if state_id == 21919 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#south: South::Tall,
                r#up: true,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::Tall,
            });
        }
        if state_id == 21758 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::None,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::None,
                r#waterlogged: true,
            });
        }
        if state_id == 21983 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::None,
            });
        }
        if state_id == 21743 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::None,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 22022 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: true,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 21861 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#north: North::Low,
                r#west: West::Low,
            });
        }
        if state_id == 21765 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#up: false,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 21948 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21963 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::None,
            });
        }
        if state_id == 21982 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
                r#west: West::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 21834 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: false,
                r#north: North::None,
            });
        }
        if state_id == 22010 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 21747 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#waterlogged: true,
                r#south: South::None,
                r#east: East::None,
                r#north: North::Low,
                r#up: true,
            });
        }
        if state_id == 21984 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#up: false,
                r#south: South::Low,
                r#east: East::Tall,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 21923 {
            return Some(BlackstoneWall {
                r#up: false,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 21878 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#south: South::Tall,
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 21847 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::Tall,
                r#north: North::None,
                r#south: South::Tall,
                r#east: East::Low,
                r#waterlogged: false,
            });
        }
        if state_id == 21725 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::None,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 21752 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#waterlogged: true,
                r#up: false,
                r#east: East::None,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 21881 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 21839 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 21903 {
            return Some(BlackstoneWall {
                r#up: true,
                r#south: South::Low,
                r#north: North::Tall,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 21868 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Low,
            });
        }
        if state_id == 21730 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#north: North::None,
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21733 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
                r#up: false,
            });
        }
        if state_id == 21970 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#north: North::Low,
                r#waterlogged: true,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
            });
        }
        if state_id == 21710 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#east: East::None,
                r#south: South::None,
                r#north: North::None,
                r#up: true,
                r#west: West::None,
            });
        }
        if state_id == 21895 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::Tall,
                r#south: South::None,
                r#north: North::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 21978 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21757 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#east: East::None,
                r#west: West::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21805 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 21800 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::Low,
                r#waterlogged: true,
                r#west: West::None,
                r#up: false,
                r#east: East::None,
            });
        }
        if state_id == 21837 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
            });
        }
        if state_id == 21918 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#south: South::Tall,
                r#waterlogged: false,
                r#east: East::Low,
            });
        }
        if state_id == 21793 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::None,
                r#up: false,
                r#west: West::Tall,
            });
        }
        if state_id == 21938 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 22020 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 21795 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#south: South::Low,
                r#north: North::Tall,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
            });
        }
        if state_id == 21734 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#west: West::None,
                r#waterlogged: true,
                r#south: South::Tall,
            });
        }
        if state_id == 21810 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
            });
        }
        if state_id == 21712 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 21829 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::None,
            });
        }
        if state_id == 21826 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#south: South::None,
            });
        }
        if state_id == 21760 {
            return Some(BlackstoneWall {
                r#up: true,
                r#north: North::Low,
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 21860 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#west: West::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
            });
        }
        if state_id == 21872 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#west: West::None,
                r#up: false,
                r#waterlogged: true,
                r#south: South::Low,
                r#east: East::Low,
            });
        }
        if state_id == 21880 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 21977 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#west: West::None,
                r#waterlogged: false,
                r#up: true,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 21714 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::None,
                r#north: North::None,
                r#south: South::None,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 21717 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::None,
            });
        }
        if state_id == 21818 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::None,
                r#south: South::None,
            });
        }
        if state_id == 21990 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#south: South::Tall,
                r#east: East::Tall,
                r#waterlogged: false,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 21954 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#east: East::Tall,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21802 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#north: North::Tall,
                r#east: East::None,
                r#up: false,
                r#south: South::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 21744 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#west: West::Low,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#up: false,
            });
        }
        if state_id == 21989 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: false,
                r#east: East::Tall,
                r#south: South::Tall,
                r#west: West::None,
                r#north: North::Low,
            });
        }
        if state_id == 21798 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21955 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#north: North::None,
                r#waterlogged: false,
                r#up: true,
                r#east: East::Tall,
                r#south: South::Tall,
            });
        }
        if state_id == 21840 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#east: East::Low,
                r#south: South::Low,
                r#west: West::Low,
                r#up: false,
                r#waterlogged: false,
            });
        }
        if state_id == 21946 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::Tall,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 21961 {
            return Some(BlackstoneWall {
                r#up: false,
                r#north: North::None,
                r#waterlogged: false,
                r#south: South::Tall,
                r#east: East::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 21746 {
            return Some(BlackstoneWall {
                r#up: true,
                r#east: East::None,
                r#south: South::None,
                r#west: West::None,
                r#north: North::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 21855 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
            });
        }
        if state_id == 21820 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#up: true,
                r#south: South::None,
                r#east: East::Low,
                r#north: North::None,
                r#west: West::Tall,
            });
        }
        if state_id == 21953 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::None,
                r#up: true,
                r#south: South::Tall,
                r#east: East::Tall,
            });
        }
        if state_id == 21803 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#south: South::Low,
                r#east: East::None,
            });
        }
        if state_id == 21728 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#south: South::Low,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::None,
                r#up: false,
            });
        }
        if state_id == 21748 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#east: East::None,
                r#waterlogged: true,
                r#up: true,
                r#west: West::Tall,
                r#north: North::Low,
            });
        }
        if state_id == 21941 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#up: true,
                r#north: North::None,
                r#waterlogged: false,
                r#east: East::Tall,
                r#west: West::None,
            });
        }
        if state_id == 21962 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::None,
                r#south: South::None,
                r#north: North::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 22031 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#up: false,
                r#south: South::Tall,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
            });
        }
        if state_id == 21727 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#north: North::None,
                r#west: West::Tall,
                r#east: East::None,
                r#south: South::Low,
                r#up: true,
            });
        }
        if state_id == 21783 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#south: South::None,
                r#up: true,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::Low,
            });
        }
        if state_id == 21998 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#east: East::Tall,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 21762 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#south: South::Low,
                r#east: East::None,
                r#up: true,
                r#north: North::Low,
            });
        }
        if state_id == 21827 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 21867 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 21892 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::None,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 21831 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 21981 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#north: North::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::Low,
                r#west: West::Low,
            });
        }
        if state_id == 21838 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: true,
                r#east: East::Low,
                r#west: West::Tall,
                r#north: North::None,
            });
        }
        if state_id == 22008 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#waterlogged: false,
                r#west: West::Low,
                r#up: false,
                r#east: East::Tall,
                r#south: South::None,
            });
        }
        if state_id == 21723 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::None,
                r#up: true,
                r#south: South::Low,
                r#north: North::None,
            });
        }
        if state_id == 21912 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 22023 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#east: East::Tall,
                r#up: true,
                r#west: West::Low,
                r#north: North::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 21739 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#up: true,
                r#waterlogged: false,
                r#north: North::None,
                r#south: South::Tall,
                r#west: West::Tall,
            });
        }
        if state_id == 22001 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#up: true,
                r#east: East::Tall,
                r#south: South::None,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 21819 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#north: North::None,
                r#south: South::None,
                r#east: East::Low,
                r#up: true,
                r#west: West::Low,
            });
        }
        if state_id == 21905 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#west: West::None,
                r#waterlogged: false,
            });
        }
        if state_id == 21843 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: true,
                r#east: East::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21813 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#north: North::Tall,
                r#west: West::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 21777 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#waterlogged: true,
                r#west: West::Low,
                r#up: false,
                r#north: North::Low,
                r#south: South::Tall,
            });
        }
        if state_id == 21900 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
                r#waterlogged: false,
                r#south: South::None,
                r#east: East::Low,
            });
        }
        if state_id == 21971 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#west: West::None,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 21841 {
            return Some(BlackstoneWall {
                r#north: North::None,
                r#up: false,
                r#west: West::Tall,
                r#east: East::Low,
                r#waterlogged: false,
                r#south: South::Low,
            });
        }
        if state_id == 21890 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::Tall,
                r#up: true,
                r#waterlogged: true,
                r#south: South::None,
                r#west: West::None,
            });
        }
        if state_id == 21869 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#south: South::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::None,
            });
        }
        if state_id == 21759 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
                r#south: South::Low,
                r#north: North::Low,
            });
        }
        if state_id == 22003 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#waterlogged: false,
                r#south: South::None,
                r#west: West::Tall,
                r#north: North::Tall,
                r#up: true,
            });
        }
        if state_id == 21911 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#up: false,
                r#waterlogged: false,
                r#west: West::None,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 21936 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 21909 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
                r#south: South::Low,
            });
        }
        if state_id == 21735 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: true,
            });
        }
        if state_id == 21846 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 21887 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::None,
                r#south: South::Tall,
                r#up: false,
                r#north: North::Low,
                r#east: East::Low,
            });
        }
        if state_id == 21902 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#up: true,
                r#south: South::Low,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::None,
            });
        }
        if state_id == 21976 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#east: East::Tall,
                r#north: North::Low,
                r#up: true,
                r#south: South::Low,
                r#waterlogged: true,
            });
        }
        if state_id == 21921 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#south: South::Tall,
                r#up: false,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::Tall,
            });
        }
        if state_id == 21794 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#up: true,
                r#waterlogged: true,
                r#east: East::None,
                r#north: North::Tall,
                r#west: West::None,
            });
        }
        if state_id == 21972 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#north: North::Low,
            });
        }
        if state_id == 21975 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Low,
                r#east: East::Tall,
            });
        }
        if state_id == 21750 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#west: West::Low,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::None,
                r#up: true,
            });
        }
        if state_id == 21859 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::Low,
                r#up: true,
                r#waterlogged: false,
                r#west: West::Tall,
                r#south: South::None,
            });
        }
        if state_id == 21862 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: false,
            });
        }
        if state_id == 21993 {
            return Some(BlackstoneWall {
                r#up: false,
                r#north: North::Low,
                r#west: West::Low,
                r#south: South::Tall,
                r#waterlogged: true,
                r#east: East::Tall,
            });
        }
        if state_id == 21927 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#waterlogged: true,
                r#south: South::None,
                r#north: North::None,
                r#west: West::Low,
                r#up: true,
            });
        }
        if state_id == 21785 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
            });
        }
        if state_id == 21893 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#west: West::None,
                r#east: East::Low,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21944 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#north: North::None,
                r#south: South::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: true,
            });
        }
        if state_id == 21825 {
            return Some(BlackstoneWall {
                r#up: false,
                r#waterlogged: true,
                r#west: West::Low,
                r#east: East::Low,
                r#north: North::None,
                r#south: South::None,
            });
        }
        if state_id == 21876 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#north: North::Low,
                r#west: West::Low,
                r#east: East::Low,
                r#up: false,
                r#south: South::Low,
            });
        }
        if state_id == 21999 {
            return Some(BlackstoneWall {
                r#south: South::None,
                r#north: North::Tall,
                r#up: true,
                r#west: West::Low,
                r#east: East::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 21784 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#north: North::Tall,
                r#up: true,
                r#south: South::None,
                r#west: West::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 21738 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#up: true,
                r#east: East::None,
                r#waterlogged: false,
                r#west: West::Low,
                r#north: North::None,
            });
        }
        if state_id == 21804 {
            return Some(BlackstoneWall {
                r#east: East::None,
                r#waterlogged: false,
                r#south: South::Low,
                r#west: West::Low,
                r#north: North::Tall,
                r#up: false,
            });
        }
        if state_id == 21724 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
                r#north: North::None,
                r#east: East::None,
                r#south: South::Low,
            });
        }
        if state_id == 21766 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#north: North::Low,
                r#east: East::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21776 {
            return Some(BlackstoneWall {
                r#west: West::None,
                r#up: false,
                r#east: East::None,
                r#north: North::Low,
                r#south: South::Tall,
                r#waterlogged: true,
            });
        }
        if state_id == 21842 {
            return Some(BlackstoneWall {
                r#up: true,
                r#waterlogged: true,
                r#north: North::None,
                r#west: West::None,
                r#south: South::Tall,
                r#east: East::Low,
            });
        }
        if state_id == 21844 {
            return Some(BlackstoneWall {
                r#east: East::Low,
                r#north: North::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21873 {
            return Some(BlackstoneWall {
                r#north: North::Low,
                r#south: South::Low,
                r#up: false,
                r#east: East::Low,
                r#waterlogged: true,
                r#west: West::Low,
            });
        }
        if state_id == 21808 {
            return Some(BlackstoneWall {
                r#west: West::Tall,
                r#north: North::Tall,
                r#east: East::None,
                r#south: South::Tall,
                r#up: true,
                r#waterlogged: true,
            });
        }
        if state_id == 21870 {
            return Some(BlackstoneWall {
                r#west: West::Low,
                r#waterlogged: false,
                r#south: South::Low,
                r#north: North::Low,
                r#up: true,
                r#east: East::Low,
            });
        }
        if state_id == 21901 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: false,
                r#east: East::Low,
                r#west: West::Tall,
            });
        }
        if state_id == 21940 {
            return Some(BlackstoneWall {
                r#waterlogged: true,
                r#west: West::Tall,
                r#up: true,
                r#east: East::Tall,
                r#north: North::None,
                r#south: South::Low,
            });
        }
        if state_id == 21947 {
            return Some(BlackstoneWall {
                r#south: South::Low,
                r#west: West::None,
                r#north: North::None,
                r#up: false,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 21965 {
            return Some(BlackstoneWall {
                r#up: true,
                r#west: West::None,
                r#north: North::Low,
                r#south: South::None,
                r#east: East::Tall,
                r#waterlogged: false,
            });
        }
        if state_id == 21979 {
            return Some(BlackstoneWall {
                r#east: East::Tall,
                r#north: North::Low,
                r#south: South::Low,
                r#west: West::Tall,
                r#up: true,
                r#waterlogged: false,
            });
        }
        if state_id == 21996 {
            return Some(BlackstoneWall {
                r#south: South::Tall,
                r#north: North::Low,
                r#east: East::Tall,
                r#up: false,
                r#waterlogged: false,
                r#west: West::Low,
            });
        }
        if state_id == 22006 {
            return Some(BlackstoneWall {
                r#north: North::Tall,
                r#east: East::Tall,
                r#south: South::None,
                r#up: false,
                r#waterlogged: true,
                r#west: West::Tall,
            });
        }
        if state_id == 21917 {
            return Some(BlackstoneWall {
                r#waterlogged: false,
                r#up: true,
                r#north: North::Tall,
                r#east: East::Low,
                r#south: South::Tall,
                r#west: West::None,
            });
        }
        return None;
    }
}

