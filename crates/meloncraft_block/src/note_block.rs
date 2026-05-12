use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NoteBlock {
    pub r#instrument: Instrument,
    pub note: i32,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instrument {
    Harp,
    Basedrum,
    Snare,
    Hat,
    Bass,
    Flute,
    Bell,
    Guitar,
    Chime,
    Xylophone,
    IronXylophone,
    CowBell,
    Didgeridoo,
    Bit,
    Banjo,
    Pling,
    Zombie,
    Skeleton,
    Creeper,
    Dragon,
    WitherSkeleton,
    Piglin,
    CustomHead,
}

impl BlockState for NoteBlock {
    fn to_id(&self) -> i32 {
        if self.r#instrument == Instrument::Dragon && self.r#powered == true && self.r#note == 14 { return 1559; }
        if self.r#instrument == Instrument::Guitar && self.r#powered == false && self.r#note == 17 { return 966; }
        if self.r#instrument == Instrument::Zombie && self.r#note == 17 && self.r#powered == false { return 1416; }
        if self.r#instrument == Instrument::Chime && self.r#powered == true && self.r#note == 4 { return 989; }
        if self.r#powered == true && self.r#instrument == Instrument::Chime && self.r#note == 9 { return 999; }
        if self.r#powered == true && self.r#note == 24 && self.r#instrument == Instrument::Zombie { return 1429; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 9 && self.r#powered == true { return 1699; }
        if self.r#instrument == Instrument::Harp && self.r#note == 16 && self.r#powered == true { return 613; }
        if self.r#powered == true && self.r#note == 24 && self.r#instrument == Instrument::WitherSkeleton { return 1629; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 1 && self.r#powered == true { return 1683; }
        if self.r#powered == true && self.r#instrument == Instrument::Snare && self.r#note == 13 { return 707; }
        if self.r#powered == true && self.r#instrument == Instrument::Didgeridoo && self.r#note == 22 { return 1225; }
        if self.r#note == 11 && self.r#instrument == Instrument::Creeper && self.r#powered == true { return 1503; }
        if self.r#instrument == Instrument::Guitar && self.r#note == 13 && self.r#powered == true { return 957; }
        if self.r#instrument == Instrument::Chime && self.r#note == 22 && self.r#powered == false { return 1026; }
        if self.r#powered == true && self.r#note == 22 && self.r#instrument == Instrument::Pling { return 1375; }
        if self.r#note == 0 && self.r#powered == true && self.r#instrument == Instrument::Banjo { return 1281; }
        if self.r#instrument == Instrument::Creeper && self.r#powered == true && self.r#note == 14 { return 1509; }
        if self.r#note == 17 && self.r#powered == true && self.r#instrument == Instrument::Dragon { return 1565; }
        if self.r#note == 4 && self.r#powered == true && self.r#instrument == Instrument::WitherSkeleton { return 1589; }
        if self.r#instrument == Instrument::Bass && self.r#note == 12 && self.r#powered == true { return 805; }
        if self.r#instrument == Instrument::Xylophone && self.r#note == 15 && self.r#powered == true { return 1061; }
        if self.r#powered == false && self.r#instrument == Instrument::Bit && self.r#note == 18 { return 1268; }
        if self.r#powered == true && self.r#instrument == Instrument::WitherSkeleton && self.r#note == 17 { return 1615; }
        if self.r#instrument == Instrument::Guitar && self.r#powered == true && self.r#note == 19 { return 969; }
        if self.r#instrument == Instrument::Skeleton && self.r#note == 17 && self.r#powered == true { return 1465; }
        if self.r#powered == false && self.r#note == 21 && self.r#instrument == Instrument::WitherSkeleton { return 1624; }
        if self.r#powered == false && self.r#instrument == Instrument::Snare && self.r#note == 18 { return 718; }
        if self.r#instrument == Instrument::Xylophone && self.r#note == 11 && self.r#powered == true { return 1053; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#note == 18 && self.r#powered == true { return 1617; }
        if self.r#note == 12 && self.r#powered == true && self.r#instrument == Instrument::CustomHead { return 1705; }
        if self.r#note == 21 && self.r#powered == false && self.r#instrument == Instrument::Harp { return 624; }
        if self.r#instrument == Instrument::Bit && self.r#note == 3 && self.r#powered == true { return 1237; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 21 && self.r#powered == true { return 1723; }
        if self.r#powered == false && self.r#note == 15 && self.r#instrument == Instrument::Piglin { return 1662; }
        if self.r#instrument == Instrument::Zombie && self.r#note == 7 && self.r#powered == false { return 1396; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 5 && self.r#powered == false { return 1492; }
        if self.r#note == 23 && self.r#powered == true && self.r#instrument == Instrument::Dragon { return 1577; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 23 && self.r#powered == false { return 1128; }
        if self.r#powered == true && self.r#note == 21 && self.r#instrument == Instrument::Bass { return 823; }
        if self.r#instrument == Instrument::Skeleton && self.r#powered == true && self.r#note == 10 { return 1451; }
        if self.r#note == 0 && self.r#powered == true && self.r#instrument == Instrument::Chime { return 981; }
        if self.r#note == 24 && self.r#powered == true && self.r#instrument == Instrument::Skeleton { return 1479; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 9 && self.r#powered == true { return 649; }
        if self.r#note == 9 && self.r#instrument == Instrument::Zombie && self.r#powered == false { return 1400; }
        if self.r#instrument == Instrument::IronXylophone && self.r#powered == false && self.r#note == 21 { return 1124; }
        if self.r#powered == true && self.r#note == 8 && self.r#instrument == Instrument::Piglin { return 1647; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 23 && self.r#powered == false { return 1678; }
        if self.r#instrument == Instrument::CustomHead && self.r#powered == false && self.r#note == 0 { return 1682; }
        if self.r#note == 2 && self.r#powered == false && self.r#instrument == Instrument::Pling { return 1336; }
        if self.r#powered == true && self.r#instrument == Instrument::Bass && self.r#note == 8 { return 797; }
        if self.r#powered == true && self.r#instrument == Instrument::Chime && self.r#note == 19 { return 1019; }
        if self.r#powered == true && self.r#instrument == Instrument::Chime && self.r#note == 18 { return 1017; }
        if self.r#instrument == Instrument::Guitar && self.r#powered == false && self.r#note == 23 { return 978; }
        if self.r#powered == true && self.r#note == 16 && self.r#instrument == Instrument::Banjo { return 1313; }
        if self.r#instrument == Instrument::Xylophone && self.r#note == 17 && self.r#powered == false { return 1066; }
        if self.r#powered == false && self.r#instrument == Instrument::Bass && self.r#note == 17 { return 816; }
        if self.r#powered == true && self.r#instrument == Instrument::Basedrum && self.r#note == 6 { return 643; }
        if self.r#instrument == Instrument::Bit && self.r#note == 17 && self.r#powered == true { return 1265; }
        if self.r#instrument == Instrument::Zombie && self.r#powered == true && self.r#note == 19 { return 1419; }
        if self.r#powered == true && self.r#instrument == Instrument::Bass && self.r#note == 9 { return 799; }
        if self.r#instrument == Instrument::Bass && self.r#note == 10 && self.r#powered == false { return 802; }
        if self.r#instrument == Instrument::Bit && self.r#powered == true && self.r#note == 9 { return 1249; }
        if self.r#note == 5 && self.r#instrument == Instrument::IronXylophone && self.r#powered == false { return 1092; }
        if self.r#instrument == Instrument::CustomHead && self.r#powered == false && self.r#note == 9 { return 1700; }
        if self.r#instrument == Instrument::Banjo && self.r#powered == false && self.r#note == 3 { return 1288; }
        if self.r#instrument == Instrument::Harp && self.r#powered == false && self.r#note == 7 { return 596; }
        if self.r#powered == true && self.r#instrument == Instrument::Xylophone && self.r#note == 7 { return 1045; }
        if self.r#note == 1 && self.r#powered == false && self.r#instrument == Instrument::Guitar { return 934; }
        if self.r#powered == false && self.r#note == 21 && self.r#instrument == Instrument::Zombie { return 1424; }
        if self.r#note == 13 && self.r#powered == true && self.r#instrument == Instrument::Pling { return 1357; }
        if self.r#note == 0 && self.r#instrument == Instrument::Creeper && self.r#powered == false { return 1482; }
        if self.r#powered == true && self.r#note == 24 && self.r#instrument == Instrument::Hat { return 779; }
        if self.r#instrument == Instrument::Hat && self.r#powered == true && self.r#note == 20 { return 771; }
        if self.r#instrument == Instrument::Skeleton && self.r#note == 8 && self.r#powered == false { return 1448; }
        if self.r#powered == true && self.r#instrument == Instrument::Guitar && self.r#note == 3 { return 937; }
        if self.r#note == 24 && self.r#powered == true && self.r#instrument == Instrument::Xylophone { return 1079; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#powered == true && self.r#note == 5 { return 1191; }
        if self.r#instrument == Instrument::Pling && self.r#note == 17 && self.r#powered == true { return 1365; }
        if self.r#note == 20 && self.r#powered == false && self.r#instrument == Instrument::Zombie { return 1422; }
        if self.r#note == 21 && self.r#instrument == Instrument::Pling && self.r#powered == false { return 1374; }
        if self.r#note == 11 && self.r#powered == true && self.r#instrument == Instrument::Basedrum { return 653; }
        if self.r#note == 12 && self.r#powered == true && self.r#instrument == Instrument::Dragon { return 1555; }
        if self.r#note == 0 && self.r#powered == false && self.r#instrument == Instrument::Snare { return 682; }
        if self.r#note == 15 && self.r#powered == false && self.r#instrument == Instrument::Flute { return 862; }
        if self.r#note == 4 && self.r#instrument == Instrument::Creeper && self.r#powered == false { return 1490; }
        if self.r#powered == true && self.r#instrument == Instrument::Dragon && self.r#note == 0 { return 1531; }
        if self.r#note == 19 && self.r#powered == true && self.r#instrument == Instrument::Flute { return 869; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#powered == false && self.r#note == 2 { return 1186; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 14 && self.r#powered == false { return 1310; }
        if self.r#instrument == Instrument::CowBell && self.r#powered == false && self.r#note == 14 { return 1160; }
        if self.r#note == 16 && self.r#powered == true && self.r#instrument == Instrument::Guitar { return 963; }
        if self.r#instrument == Instrument::Bit && self.r#powered == true && self.r#note == 4 { return 1239; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 13 && self.r#powered == false { return 1658; }
        if self.r#note == 6 && self.r#instrument == Instrument::CustomHead && self.r#powered == true { return 1693; }
        if self.r#instrument == Instrument::Hat && self.r#note == 19 && self.r#powered == true { return 769; }
        if self.r#note == 9 && self.r#powered == false && self.r#instrument == Instrument::Bass { return 800; }
        if self.r#powered == true && self.r#instrument == Instrument::Basedrum && self.r#note == 15 { return 661; }
        if self.r#powered == true && self.r#note == 1 && self.r#instrument == Instrument::Bell { return 883; }
        if self.r#powered == false && self.r#instrument == Instrument::Creeper && self.r#note == 12 { return 1506; }
        if self.r#instrument == Instrument::Harp && self.r#note == 12 && self.r#powered == true { return 605; }
        if self.r#instrument == Instrument::Dragon && self.r#note == 13 && self.r#powered == true { return 1557; }
        if self.r#note == 19 && self.r#instrument == Instrument::Bit && self.r#powered == false { return 1270; }
        if self.r#instrument == Instrument::Harp && self.r#powered == false && self.r#note == 9 { return 600; }
        if self.r#instrument == Instrument::Basedrum && self.r#powered == false && self.r#note == 11 { return 654; }
        if self.r#instrument == Instrument::Bell && self.r#note == 19 && self.r#powered == true { return 919; }
        if self.r#instrument == Instrument::Chime && self.r#powered == true && self.r#note == 17 { return 1015; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#note == 24 && self.r#powered == true { return 1229; }
        if self.r#note == 11 && self.r#instrument == Instrument::Zombie && self.r#powered == true { return 1403; }
        if self.r#powered == true && self.r#note == 17 && self.r#instrument == Instrument::Hat { return 765; }
        if self.r#instrument == Instrument::Harp && self.r#note == 12 && self.r#powered == false { return 606; }
        if self.r#note == 24 && self.r#instrument == Instrument::Guitar && self.r#powered == false { return 980; }
        if self.r#note == 6 && self.r#powered == false && self.r#instrument == Instrument::CowBell { return 1144; }
        if self.r#note == 12 && self.r#powered == true && self.r#instrument == Instrument::Guitar { return 955; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#note == 1 && self.r#powered == false { return 1184; }
        if self.r#note == 4 && self.r#instrument == Instrument::Banjo && self.r#powered == false { return 1290; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 18 && self.r#powered == true { return 1317; }
        if self.r#powered == true && self.r#instrument == Instrument::Pling && self.r#note == 9 { return 1349; }
        if self.r#powered == false && self.r#instrument == Instrument::Xylophone && self.r#note == 13 { return 1058; }
        if self.r#powered == false && self.r#instrument == Instrument::Bell && self.r#note == 4 { return 890; }
        if self.r#powered == true && self.r#note == 22 && self.r#instrument == Instrument::Xylophone { return 1075; }
        if self.r#powered == false && self.r#note == 2 && self.r#instrument == Instrument::Banjo { return 1286; }
        if self.r#note == 12 && self.r#instrument == Instrument::Zombie && self.r#powered == false { return 1406; }
        if self.r#instrument == Instrument::Hat && self.r#powered == true && self.r#note == 6 { return 743; }
        if self.r#powered == true && self.r#note == 2 && self.r#instrument == Instrument::Piglin { return 1635; }
        if self.r#powered == true && self.r#note == 21 && self.r#instrument == Instrument::Flute { return 873; }
        if self.r#powered == true && self.r#instrument == Instrument::Xylophone && self.r#note == 18 { return 1067; }
        if self.r#instrument == Instrument::Flute && self.r#note == 18 && self.r#powered == false { return 868; }
        if self.r#instrument == Instrument::Snare && self.r#note == 3 && self.r#powered == false { return 688; }
        if self.r#instrument == Instrument::Chime && self.r#powered == true && self.r#note == 23 { return 1027; }
        if self.r#instrument == Instrument::Xylophone && self.r#powered == false && self.r#note == 4 { return 1040; }
        if self.r#powered == false && self.r#instrument == Instrument::Zombie && self.r#note == 0 { return 1382; }
        if self.r#powered == true && self.r#instrument == Instrument::Chime && self.r#note == 8 { return 997; }
        if self.r#powered == false && self.r#instrument == Instrument::CustomHead && self.r#note == 16 { return 1714; }
        if self.r#instrument == Instrument::IronXylophone && self.r#powered == false && self.r#note == 20 { return 1122; }
        if self.r#powered == false && self.r#note == 10 && self.r#instrument == Instrument::Basedrum { return 652; }
        if self.r#instrument == Instrument::Flute && self.r#note == 6 && self.r#powered == true { return 843; }
        if self.r#note == 20 && self.r#powered == true && self.r#instrument == Instrument::Bell { return 921; }
        if self.r#instrument == Instrument::Bass && self.r#note == 5 && self.r#powered == false { return 792; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#note == 0 && self.r#powered == true { return 1581; }
        if self.r#powered == true && self.r#instrument == Instrument::Guitar && self.r#note == 6 { return 943; }
        if self.r#note == 14 && self.r#instrument == Instrument::Bell && self.r#powered == true { return 909; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 9 && self.r#powered == false { return 1100; }
        if self.r#note == 18 && self.r#instrument == Instrument::Xylophone && self.r#powered == false { return 1068; }
        if self.r#note == 9 && self.r#instrument == Instrument::WitherSkeleton && self.r#powered == true { return 1599; }
        if self.r#powered == false && self.r#note == 14 && self.r#instrument == Instrument::Xylophone { return 1060; }
        if self.r#note == 14 && self.r#instrument == Instrument::Bell && self.r#powered == false { return 910; }
        if self.r#instrument == Instrument::Guitar && self.r#note == 0 && self.r#powered == true { return 931; }
        if self.r#note == 6 && self.r#instrument == Instrument::CowBell && self.r#powered == true { return 1143; }
        if self.r#powered == true && self.r#instrument == Instrument::IronXylophone && self.r#note == 24 { return 1129; }
        if self.r#instrument == Instrument::Pling && self.r#note == 12 && self.r#powered == true { return 1355; }
        if self.r#powered == false && self.r#instrument == Instrument::Harp && self.r#note == 17 { return 616; }
        if self.r#note == 5 && self.r#powered == false && self.r#instrument == Instrument::Bell { return 892; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#powered == true && self.r#note == 3 { return 1187; }
        if self.r#powered == false && self.r#instrument == Instrument::CustomHead && self.r#note == 12 { return 1706; }
        if self.r#powered == false && self.r#note == 6 && self.r#instrument == Instrument::Piglin { return 1644; }
        if self.r#note == 12 && self.r#powered == true && self.r#instrument == Instrument::Snare { return 705; }
        if self.r#powered == true && self.r#note == 19 && self.r#instrument == Instrument::Banjo { return 1319; }
        if self.r#powered == true && self.r#instrument == Instrument::Banjo && self.r#note == 11 { return 1303; }
        if self.r#note == 15 && self.r#instrument == Instrument::Dragon && self.r#powered == true { return 1561; }
        if self.r#note == 8 && self.r#instrument == Instrument::Piglin && self.r#powered == false { return 1648; }
        if self.r#powered == false && self.r#note == 7 && self.r#instrument == Instrument::Creeper { return 1496; }
        if self.r#note == 8 && self.r#powered == true && self.r#instrument == Instrument::Didgeridoo { return 1197; }
        if self.r#instrument == Instrument::Hat && self.r#note == 10 && self.r#powered == true { return 751; }
        if self.r#powered == false && self.r#instrument == Instrument::Skeleton && self.r#note == 0 { return 1432; }
        if self.r#note == 23 && self.r#instrument == Instrument::Skeleton && self.r#powered == true { return 1477; }
        if self.r#note == 13 && self.r#instrument == Instrument::IronXylophone && self.r#powered == false { return 1108; }
        if self.r#instrument == Instrument::Xylophone && self.r#note == 0 && self.r#powered == true { return 1031; }
        if self.r#powered == true && self.r#instrument == Instrument::Didgeridoo && self.r#note == 10 { return 1201; }
        if self.r#powered == false && self.r#instrument == Instrument::Piglin && self.r#note == 19 { return 1670; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#note == 12 && self.r#powered == false { return 1606; }
        if self.r#powered == true && self.r#instrument == Instrument::Snare && self.r#note == 8 { return 697; }
        if self.r#powered == true && self.r#note == 3 && self.r#instrument == Instrument::WitherSkeleton { return 1587; }
        if self.r#note == 2 && self.r#powered == false && self.r#instrument == Instrument::Piglin { return 1636; }
        if self.r#powered == true && self.r#note == 15 && self.r#instrument == Instrument::IronXylophone { return 1111; }
        if self.r#powered == false && self.r#instrument == Instrument::Didgeridoo && self.r#note == 22 { return 1226; }
        if self.r#note == 14 && self.r#powered == true && self.r#instrument == Instrument::Guitar { return 959; }
        if self.r#note == 23 && self.r#powered == false && self.r#instrument == Instrument::Basedrum { return 678; }
        if self.r#powered == true && self.r#note == 2 && self.r#instrument == Instrument::Hat { return 735; }
        if self.r#note == 11 && self.r#instrument == Instrument::Hat && self.r#powered == false { return 754; }
        if self.r#note == 23 && self.r#powered == false && self.r#instrument == Instrument::CowBell { return 1178; }
        if self.r#instrument == Instrument::Bass && self.r#powered == true && self.r#note == 17 { return 815; }
        if self.r#instrument == Instrument::Pling && self.r#note == 5 && self.r#powered == true { return 1341; }
        if self.r#note == 2 && self.r#instrument == Instrument::Bass && self.r#powered == true { return 785; }
        if self.r#powered == true && self.r#note == 1 && self.r#instrument == Instrument::Flute { return 833; }
        if self.r#note == 1 && self.r#instrument == Instrument::Zombie && self.r#powered == false { return 1384; }
        if self.r#powered == true && self.r#note == 2 && self.r#instrument == Instrument::Bell { return 885; }
        if self.r#note == 18 && self.r#powered == true && self.r#instrument == Instrument::Flute { return 867; }
        if self.r#note == 11 && self.r#powered == false && self.r#instrument == Instrument::Guitar { return 954; }
        if self.r#instrument == Instrument::Chime && self.r#note == 13 && self.r#powered == false { return 1008; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 2 && self.r#powered == false { return 1086; }
        if self.r#note == 8 && self.r#powered == false && self.r#instrument == Instrument::Harp { return 598; }
        if self.r#note == 14 && self.r#instrument == Instrument::IronXylophone && self.r#powered == true { return 1109; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 22 && self.r#powered == false { return 1326; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 24 && self.r#powered == false { return 1530; }
        if self.r#powered == true && self.r#instrument == Instrument::Pling && self.r#note == 18 { return 1367; }
        if self.r#note == 19 && self.r#powered == false && self.r#instrument == Instrument::WitherSkeleton { return 1620; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 10 && self.r#powered == true { return 651; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 19 && self.r#powered == true { return 1119; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 20 && self.r#powered == false { return 1322; }
        if self.r#powered == true && self.r#note == 17 && self.r#instrument == Instrument::Flute { return 865; }
        if self.r#powered == false && self.r#instrument == Instrument::Flute && self.r#note == 0 { return 832; }
        if self.r#note == 7 && self.r#powered == false && self.r#instrument == Instrument::Bell { return 896; }
        if self.r#note == 15 && self.r#powered == true && self.r#instrument == Instrument::CowBell { return 1161; }
        if self.r#powered == true && self.r#note == 23 && self.r#instrument == Instrument::Pling { return 1377; }
        if self.r#note == 8 && self.r#instrument == Instrument::IronXylophone && self.r#powered == false { return 1098; }
        if self.r#instrument == Instrument::Pling && self.r#powered == true && self.r#note == 21 { return 1373; }
        if self.r#note == 5 && self.r#powered == false && self.r#instrument == Instrument::CustomHead { return 1692; }
        if self.r#instrument == Instrument::Dragon && self.r#note == 20 && self.r#powered == true { return 1571; }
        if self.r#powered == false && self.r#instrument == Instrument::Basedrum && self.r#note == 1 { return 634; }
        if self.r#instrument == Instrument::CowBell && self.r#powered == false && self.r#note == 2 { return 1136; }
        if self.r#note == 5 && self.r#instrument == Instrument::Skeleton && self.r#powered == false { return 1442; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 1 && self.r#powered == true { return 1483; }
        if self.r#note == 4 && self.r#powered == true && self.r#instrument == Instrument::CustomHead { return 1689; }
        if self.r#note == 22 && self.r#instrument == Instrument::Snare && self.r#powered == true { return 725; }
        if self.r#powered == false && self.r#instrument == Instrument::Hat && self.r#note == 22 { return 776; }
        if self.r#powered == false && self.r#instrument == Instrument::Creeper && self.r#note == 10 { return 1502; }
        if self.r#powered == false && self.r#note == 22 && self.r#instrument == Instrument::Bit { return 1276; }
        if self.r#instrument == Instrument::Xylophone && self.r#note == 20 && self.r#powered == true { return 1071; }
        if self.r#powered == false && self.r#instrument == Instrument::Xylophone && self.r#note == 3 { return 1038; }
        if self.r#instrument == Instrument::Zombie && self.r#note == 15 && self.r#powered == true { return 1411; }
        if self.r#powered == true && self.r#instrument == Instrument::Xylophone && self.r#note == 21 { return 1073; }
        if self.r#note == 1 && self.r#powered == true && self.r#instrument == Instrument::Harp { return 583; }
        if self.r#instrument == Instrument::Dragon && self.r#note == 21 && self.r#powered == false { return 1574; }
        if self.r#powered == true && self.r#instrument == Instrument::IronXylophone && self.r#note == 17 { return 1115; }
        if self.r#note == 7 && self.r#powered == false && self.r#instrument == Instrument::WitherSkeleton { return 1596; }
        if self.r#powered == false && self.r#note == 22 && self.r#instrument == Instrument::IronXylophone { return 1126; }
        if self.r#note == 9 && self.r#instrument == Instrument::Skeleton && self.r#powered == false { return 1450; }
        if self.r#instrument == Instrument::Hat && self.r#note == 20 && self.r#powered == false { return 772; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 10 && self.r#powered == true { return 1101; }
        if self.r#instrument == Instrument::Bell && self.r#powered == true && self.r#note == 10 { return 901; }
        if self.r#note == 1 && self.r#powered == true && self.r#instrument == Instrument::Hat { return 733; }
        if self.r#instrument == Instrument::Pling && self.r#note == 16 && self.r#powered == true { return 1363; }
        if self.r#note == 1 && self.r#powered == false && self.r#instrument == Instrument::Bass { return 784; }
        if self.r#note == 20 && self.r#powered == true && self.r#instrument == Instrument::Bit { return 1271; }
        if self.r#note == 21 && self.r#powered == true && self.r#instrument == Instrument::Chime { return 1023; }
        if self.r#note == 2 && self.r#powered == true && self.r#instrument == Instrument::Xylophone { return 1035; }
        if self.r#instrument == Instrument::Xylophone && self.r#powered == true && self.r#note == 9 { return 1049; }
        if self.r#note == 15 && self.r#instrument == Instrument::Pling && self.r#powered == false { return 1362; }
        if self.r#note == 9 && self.r#powered == false && self.r#instrument == Instrument::Basedrum { return 650; }
        if self.r#note == 23 && self.r#powered == true && self.r#instrument == Instrument::Basedrum { return 677; }
        if self.r#powered == false && self.r#instrument == Instrument::Pling && self.r#note == 17 { return 1366; }
        if self.r#note == 7 && self.r#instrument == Instrument::Snare && self.r#powered == false { return 696; }
        if self.r#powered == true && self.r#instrument == Instrument::Banjo && self.r#note == 15 { return 1311; }
        if self.r#powered == true && self.r#instrument == Instrument::Skeleton && self.r#note == 16 { return 1463; }
        if self.r#instrument == Instrument::Basedrum && self.r#powered == true && self.r#note == 0 { return 631; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 20 && self.r#powered == false { return 1722; }
        if self.r#note == 19 && self.r#powered == false && self.r#instrument == Instrument::Creeper { return 1520; }
        if self.r#powered == true && self.r#instrument == Instrument::Bass && self.r#note == 23 { return 827; }
        if self.r#powered == false && self.r#instrument == Instrument::Banjo && self.r#note == 10 { return 1302; }
        if self.r#powered == true && self.r#instrument == Instrument::Hat && self.r#note == 9 { return 749; }
        if self.r#note == 17 && self.r#instrument == Instrument::Xylophone && self.r#powered == true { return 1065; }
        if self.r#powered == true && self.r#instrument == Instrument::Chime && self.r#note == 20 { return 1021; }
        if self.r#powered == false && self.r#instrument == Instrument::Banjo && self.r#note == 17 { return 1316; }
        if self.r#note == 8 && self.r#powered == false && self.r#instrument == Instrument::CowBell { return 1148; }
        if self.r#powered == true && self.r#instrument == Instrument::Dragon && self.r#note == 24 { return 1579; }
        if self.r#note == 23 && self.r#powered == true && self.r#instrument == Instrument::Bell { return 927; }
        if self.r#powered == false && self.r#instrument == Instrument::Snare && self.r#note == 14 { return 710; }
        if self.r#note == 13 && self.r#instrument == Instrument::Pling && self.r#powered == false { return 1358; }
        if self.r#note == 3 && self.r#instrument == Instrument::CowBell && self.r#powered == false { return 1138; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 21 && self.r#powered == false { return 1724; }
        if self.r#note == 18 && self.r#instrument == Instrument::Bit && self.r#powered == true { return 1267; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 15 && self.r#powered == true { return 1661; }
        if self.r#powered == false && self.r#note == 1 && self.r#instrument == Instrument::Snare { return 684; }
        if self.r#powered == false && self.r#instrument == Instrument::CowBell && self.r#note == 13 { return 1158; }
        if self.r#note == 20 && self.r#instrument == Instrument::Creeper && self.r#powered == false { return 1522; }
        if self.r#note == 21 && self.r#powered == true && self.r#instrument == Instrument::Hat { return 773; }
        if self.r#powered == false && self.r#instrument == Instrument::Bass && self.r#note == 14 { return 810; }
        if self.r#note == 0 && self.r#powered == false && self.r#instrument == Instrument::CowBell { return 1132; }
        if self.r#note == 16 && self.r#instrument == Instrument::Creeper && self.r#powered == false { return 1514; }
        if self.r#note == 9 && self.r#instrument == Instrument::Harp && self.r#powered == true { return 599; }
        if self.r#instrument == Instrument::Bass && self.r#note == 18 && self.r#powered == false { return 818; }
        if self.r#note == 7 && self.r#powered == false && self.r#instrument == Instrument::Pling { return 1346; }
        if self.r#powered == true && self.r#note == 16 && self.r#instrument == Instrument::Bell { return 913; }
        if self.r#note == 16 && self.r#powered == false && self.r#instrument == Instrument::Zombie { return 1414; }
        if self.r#instrument == Instrument::Dragon && self.r#note == 5 && self.r#powered == true { return 1541; }
        if self.r#powered == true && self.r#instrument == Instrument::Basedrum && self.r#note == 8 { return 647; }
        if self.r#note == 10 && self.r#powered == false && self.r#instrument == Instrument::Flute { return 852; }
        if self.r#instrument == Instrument::Guitar && self.r#note == 15 && self.r#powered == true { return 961; }
        if self.r#note == 7 && self.r#powered == true && self.r#instrument == Instrument::WitherSkeleton { return 1595; }
        if self.r#powered == false && self.r#instrument == Instrument::Bell && self.r#note == 23 { return 928; }
        if self.r#note == 3 && self.r#instrument == Instrument::CustomHead && self.r#powered == true { return 1687; }
        if self.r#instrument == Instrument::Bass && self.r#note == 10 && self.r#powered == true { return 801; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 4 && self.r#powered == true { return 1089; }
        if self.r#note == 15 && self.r#powered == false && self.r#instrument == Instrument::Snare { return 712; }
        if self.r#powered == false && self.r#note == 6 && self.r#instrument == Instrument::Xylophone { return 1044; }
        if self.r#instrument == Instrument::Bass && self.r#note == 20 && self.r#powered == false { return 822; }
        if self.r#instrument == Instrument::Pling && self.r#powered == false && self.r#note == 3 { return 1338; }
        if self.r#note == 19 && self.r#powered == true && self.r#instrument == Instrument::WitherSkeleton { return 1619; }
        if self.r#powered == false && self.r#instrument == Instrument::Zombie && self.r#note == 19 { return 1420; }
        if self.r#powered == true && self.r#instrument == Instrument::Hat && self.r#note == 4 { return 739; }
        if self.r#powered == true && self.r#note == 6 && self.r#instrument == Instrument::Didgeridoo { return 1193; }
        if self.r#note == 9 && self.r#powered == false && self.r#instrument == Instrument::Didgeridoo { return 1200; }
        if self.r#powered == false && self.r#instrument == Instrument::Harp && self.r#note == 1 { return 584; }
        if self.r#powered == false && self.r#instrument == Instrument::Dragon && self.r#note == 13 { return 1558; }
        if self.r#instrument == Instrument::Flute && self.r#note == 6 && self.r#powered == false { return 844; }
        if self.r#powered == true && self.r#instrument == Instrument::Piglin && self.r#note == 6 { return 1643; }
        if self.r#powered == true && self.r#note == 20 && self.r#instrument == Instrument::WitherSkeleton { return 1621; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#powered == true && self.r#note == 1 { return 1583; }
        if self.r#instrument == Instrument::Basedrum && self.r#powered == false && self.r#note == 3 { return 638; }
        if self.r#powered == false && self.r#instrument == Instrument::Flute && self.r#note == 20 { return 872; }
        if self.r#powered == false && self.r#instrument == Instrument::Bass && self.r#note == 11 { return 804; }
        if self.r#instrument == Instrument::Zombie && self.r#note == 2 && self.r#powered == false { return 1386; }
        if self.r#powered == false && self.r#instrument == Instrument::WitherSkeleton && self.r#note == 23 { return 1628; }
        if self.r#note == 2 && self.r#instrument == Instrument::Chime && self.r#powered == false { return 986; }
        if self.r#powered == false && self.r#note == 24 && self.r#instrument == Instrument::Pling { return 1380; }
        if self.r#powered == true && self.r#instrument == Instrument::Didgeridoo && self.r#note == 0 { return 1181; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#powered == false && self.r#note == 15 { return 1212; }
        if self.r#note == 18 && self.r#powered == true && self.r#instrument == Instrument::Zombie { return 1417; }
        if self.r#powered == false && self.r#note == 20 && self.r#instrument == Instrument::Xylophone { return 1072; }
        if self.r#note == 22 && self.r#instrument == Instrument::Piglin && self.r#powered == true { return 1675; }
        if self.r#instrument == Instrument::Bass && self.r#note == 12 && self.r#powered == false { return 806; }
        if self.r#instrument == Instrument::Pling && self.r#powered == false && self.r#note == 6 { return 1344; }
        if self.r#note == 11 && self.r#instrument == Instrument::WitherSkeleton && self.r#powered == false { return 1604; }
        if self.r#powered == true && self.r#instrument == Instrument::Bell && self.r#note == 3 { return 887; }
        if self.r#instrument == Instrument::CowBell && self.r#note == 14 && self.r#powered == true { return 1159; }
        if self.r#note == 11 && self.r#powered == false && self.r#instrument == Instrument::IronXylophone { return 1104; }
        if self.r#powered == false && self.r#instrument == Instrument::Piglin && self.r#note == 22 { return 1676; }
        if self.r#note == 11 && self.r#powered == false && self.r#instrument == Instrument::Xylophone { return 1054; }
        if self.r#note == 12 && self.r#instrument == Instrument::Xylophone && self.r#powered == true { return 1055; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 13 && self.r#powered == true { return 1107; }
        if self.r#powered == true && self.r#instrument == Instrument::Didgeridoo && self.r#note == 1 { return 1183; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#note == 12 && self.r#powered == true { return 1205; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 11 && self.r#powered == false { return 1504; }
        if self.r#powered == true && self.r#note == 0 && self.r#instrument == Instrument::Pling { return 1331; }
        if self.r#powered == true && self.r#instrument == Instrument::CowBell && self.r#note == 23 { return 1177; }
        if self.r#powered == true && self.r#instrument == Instrument::Banjo && self.r#note == 14 { return 1309; }
        if self.r#note == 4 && self.r#instrument == Instrument::Zombie && self.r#powered == true { return 1389; }
        if self.r#note == 13 && self.r#powered == true && self.r#instrument == Instrument::Chime { return 1007; }
        if self.r#powered == true && self.r#instrument == Instrument::WitherSkeleton && self.r#note == 5 { return 1591; }
        if self.r#instrument == Instrument::Bass && self.r#note == 16 && self.r#powered == true { return 813; }
        if self.r#note == 0 && self.r#powered == false && self.r#instrument == Instrument::WitherSkeleton { return 1582; }
        if self.r#note == 0 && self.r#instrument == Instrument::Harp && self.r#powered == false { return 582; }
        if self.r#instrument == Instrument::Snare && self.r#note == 22 && self.r#powered == false { return 726; }
        if self.r#powered == false && self.r#instrument == Instrument::Banjo && self.r#note == 15 { return 1312; }
        if self.r#instrument == Instrument::Bell && self.r#note == 11 && self.r#powered == false { return 904; }
        if self.r#instrument == Instrument::Harp && self.r#note == 10 && self.r#powered == false { return 602; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 3 && self.r#powered == false { return 1088; }
        if self.r#instrument == Instrument::Skeleton && self.r#note == 11 && self.r#powered == false { return 1454; }
        if self.r#instrument == Instrument::Creeper && self.r#powered == true && self.r#note == 0 { return 1481; }
        if self.r#powered == false && self.r#instrument == Instrument::Bell && self.r#note == 18 { return 918; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 23 && self.r#powered == false { return 1728; }
        if self.r#instrument == Instrument::CowBell && self.r#note == 17 && self.r#powered == true { return 1165; }
        if self.r#note == 4 && self.r#instrument == Instrument::WitherSkeleton && self.r#powered == false { return 1590; }
        if self.r#powered == true && self.r#instrument == Instrument::Pling && self.r#note == 8 { return 1347; }
        if self.r#note == 18 && self.r#instrument == Instrument::CustomHead && self.r#powered == false { return 1718; }
        if self.r#instrument == Instrument::Dragon && self.r#note == 16 && self.r#powered == true { return 1563; }
        if self.r#note == 9 && self.r#powered == false && self.r#instrument == Instrument::Guitar { return 950; }
        if self.r#powered == true && self.r#instrument == Instrument::Zombie && self.r#note == 0 { return 1381; }
        if self.r#instrument == Instrument::Piglin && self.r#powered == true && self.r#note == 7 { return 1645; }
        if self.r#instrument == Instrument::Pling && self.r#note == 10 && self.r#powered == false { return 1352; }
        if self.r#instrument == Instrument::Hat && self.r#powered == false && self.r#note == 15 { return 762; }
        if self.r#note == 6 && self.r#powered == true && self.r#instrument == Instrument::Creeper { return 1493; }
        if self.r#powered == false && self.r#note == 5 && self.r#instrument == Instrument::Banjo { return 1292; }
        if self.r#powered == false && self.r#instrument == Instrument::Bell && self.r#note == 1 { return 884; }
        if self.r#powered == false && self.r#instrument == Instrument::Flute && self.r#note == 2 { return 836; }
        if self.r#instrument == Instrument::Guitar && self.r#note == 10 && self.r#powered == true { return 951; }
        if self.r#note == 16 && self.r#instrument == Instrument::Banjo && self.r#powered == false { return 1314; }
        if self.r#powered == false && self.r#instrument == Instrument::Zombie && self.r#note == 22 { return 1426; }
        if self.r#powered == false && self.r#instrument == Instrument::Banjo && self.r#note == 24 { return 1330; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 17 && self.r#powered == true { return 1715; }
        if self.r#instrument == Instrument::Zombie && self.r#powered == false && self.r#note == 24 { return 1430; }
        if self.r#note == 13 && self.r#powered == true && self.r#instrument == Instrument::Bass { return 807; }
        if self.r#powered == false && self.r#instrument == Instrument::Bell && self.r#note == 9 { return 900; }
        if self.r#powered == true && self.r#note == 24 && self.r#instrument == Instrument::Bit { return 1279; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 16 && self.r#powered == true { return 1113; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 19 && self.r#powered == true { return 669; }
        if self.r#note == 23 && self.r#instrument == Instrument::Didgeridoo && self.r#powered == false { return 1228; }
        if self.r#instrument == Instrument::Snare && self.r#note == 19 && self.r#powered == false { return 720; }
        if self.r#note == 14 && self.r#instrument == Instrument::Zombie && self.r#powered == false { return 1410; }
        if self.r#note == 12 && self.r#powered == true && self.r#instrument == Instrument::Skeleton { return 1455; }
        if self.r#note == 18 && self.r#powered == true && self.r#instrument == Instrument::Dragon { return 1567; }
        if self.r#note == 0 && self.r#instrument == Instrument::Snare && self.r#powered == true { return 681; }
        if self.r#powered == false && self.r#instrument == Instrument::Guitar && self.r#note == 5 { return 942; }
        if self.r#powered == false && self.r#instrument == Instrument::Banjo && self.r#note == 0 { return 1282; }
        if self.r#instrument == Instrument::Zombie && self.r#powered == false && self.r#note == 3 { return 1388; }
        if self.r#instrument == Instrument::Bell && self.r#powered == true && self.r#note == 22 { return 925; }
        if self.r#powered == false && self.r#instrument == Instrument::Hat && self.r#note == 12 { return 756; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 12 && self.r#powered == false { return 656; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#note == 18 && self.r#powered == false { return 1618; }
        if self.r#powered == true && self.r#note == 3 && self.r#instrument == Instrument::Harp { return 587; }
        if self.r#instrument == Instrument::Hat && self.r#note == 21 && self.r#powered == false { return 774; }
        if self.r#instrument == Instrument::Zombie && self.r#note == 13 && self.r#powered == false { return 1408; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 21 && self.r#powered == false { return 674; }
        if self.r#instrument == Instrument::Guitar && self.r#powered == true && self.r#note == 9 { return 949; }
        if self.r#powered == true && self.r#instrument == Instrument::Basedrum && self.r#note == 16 { return 663; }
        if self.r#instrument == Instrument::Bell && self.r#powered == true && self.r#note == 7 { return 895; }
        if self.r#powered == false && self.r#instrument == Instrument::Chime && self.r#note == 8 { return 998; }
        if self.r#powered == true && self.r#note == 13 && self.r#instrument == Instrument::Flute { return 857; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 18 && self.r#powered == true { return 1717; }
        if self.r#note == 6 && self.r#powered == false && self.r#instrument == Instrument::Dragon { return 1544; }
        if self.r#note == 10 && self.r#powered == false && self.r#instrument == Instrument::Dragon { return 1552; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 24 && self.r#powered == true { return 679; }
        if self.r#instrument == Instrument::CustomHead && self.r#powered == false && self.r#note == 7 { return 1696; }
        if self.r#note == 20 && self.r#instrument == Instrument::CowBell && self.r#powered == false { return 1172; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 18 && self.r#powered == false { return 1668; }
        if self.r#note == 16 && self.r#instrument == Instrument::Harp && self.r#powered == false { return 614; }
        if self.r#note == 10 && self.r#instrument == Instrument::Flute && self.r#powered == true { return 851; }
        if self.r#instrument == Instrument::Skeleton && self.r#note == 21 && self.r#powered == false { return 1474; }
        if self.r#powered == true && self.r#instrument == Instrument::Bass && self.r#note == 14 { return 809; }
        if self.r#note == 21 && self.r#powered == false && self.r#instrument == Instrument::Creeper { return 1524; }
        if self.r#powered == false && self.r#instrument == Instrument::Bell && self.r#note == 15 { return 912; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 18 && self.r#powered == false { return 1118; }
        if self.r#instrument == Instrument::Bell && self.r#note == 3 && self.r#powered == false { return 888; }
        if self.r#note == 1 && self.r#powered == true && self.r#instrument == Instrument::Guitar { return 933; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 17 && self.r#powered == true { return 1315; }
        if self.r#note == 15 && self.r#instrument == Instrument::Flute && self.r#powered == true { return 861; }
        if self.r#note == 18 && self.r#powered == true && self.r#instrument == Instrument::Bass { return 817; }
        if self.r#powered == true && self.r#instrument == Instrument::WitherSkeleton && self.r#note == 16 { return 1613; }
        if self.r#powered == false && self.r#instrument == Instrument::Creeper && self.r#note == 8 { return 1498; }
        if self.r#instrument == Instrument::Skeleton && self.r#powered == false && self.r#note == 13 { return 1458; }
        if self.r#note == 20 && self.r#powered == true && self.r#instrument == Instrument::Flute { return 871; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#note == 16 && self.r#powered == false { return 1214; }
        if self.r#note == 14 && self.r#instrument == Instrument::Snare && self.r#powered == true { return 709; }
        if self.r#instrument == Instrument::Hat && self.r#note == 18 && self.r#powered == false { return 768; }
        if self.r#instrument == Instrument::CowBell && self.r#powered == false && self.r#note == 1 { return 1134; }
        if self.r#note == 11 && self.r#powered == false && self.r#instrument == Instrument::Didgeridoo { return 1204; }
        if self.r#powered == false && self.r#instrument == Instrument::Bit && self.r#note == 21 { return 1274; }
        if self.r#powered == true && self.r#note == 13 && self.r#instrument == Instrument::Didgeridoo { return 1207; }
        if self.r#powered == true && self.r#instrument == Instrument::Snare && self.r#note == 20 { return 721; }
        if self.r#instrument == Instrument::CowBell && self.r#note == 16 && self.r#powered == true { return 1163; }
        if self.r#note == 12 && self.r#instrument == Instrument::Didgeridoo && self.r#powered == false { return 1206; }
        if self.r#powered == true && self.r#instrument == Instrument::Pling && self.r#note == 7 { return 1345; }
        if self.r#instrument == Instrument::Chime && self.r#note == 6 && self.r#powered == true { return 993; }
        if self.r#powered == true && self.r#instrument == Instrument::Skeleton && self.r#note == 8 { return 1447; }
        if self.r#note == 11 && self.r#instrument == Instrument::Harp && self.r#powered == true { return 603; }
        if self.r#note == 20 && self.r#instrument == Instrument::Basedrum && self.r#powered == false { return 672; }
        if self.r#powered == false && self.r#instrument == Instrument::CowBell && self.r#note == 7 { return 1146; }
        if self.r#note == 1 && self.r#powered == false && self.r#instrument == Instrument::IronXylophone { return 1084; }
        if self.r#instrument == Instrument::Zombie && self.r#note == 1 && self.r#powered == true { return 1383; }
        if self.r#note == 1 && self.r#instrument == Instrument::Flute && self.r#powered == false { return 834; }
        if self.r#powered == false && self.r#instrument == Instrument::Bell && self.r#note == 24 { return 930; }
        if self.r#powered == false && self.r#instrument == Instrument::Xylophone && self.r#note == 24 { return 1080; }
        if self.r#note == 21 && self.r#instrument == Instrument::Zombie && self.r#powered == true { return 1423; }
        if self.r#instrument == Instrument::Skeleton && self.r#powered == false && self.r#note == 24 { return 1480; }
        if self.r#powered == false && self.r#instrument == Instrument::Xylophone && self.r#note == 7 { return 1046; }
        if self.r#powered == false && self.r#instrument == Instrument::Bass && self.r#note == 7 { return 796; }
        if self.r#instrument == Instrument::Zombie && self.r#note == 10 && self.r#powered == true { return 1401; }
        if self.r#instrument == Instrument::Chime && self.r#powered == true && self.r#note == 24 { return 1029; }
        if self.r#powered == false && self.r#note == 19 && self.r#instrument == Instrument::Harp { return 620; }
        if self.r#note == 9 && self.r#instrument == Instrument::Pling && self.r#powered == false { return 1350; }
        if self.r#instrument == Instrument::Hat && self.r#powered == true && self.r#note == 5 { return 741; }
        if self.r#note == 23 && self.r#instrument == Instrument::Xylophone && self.r#powered == false { return 1078; }
        if self.r#powered == true && self.r#instrument == Instrument::Pling && self.r#note == 3 { return 1337; }
        if self.r#powered == false && self.r#instrument == Instrument::Flute && self.r#note == 21 { return 874; }
        if self.r#instrument == Instrument::IronXylophone && self.r#powered == true && self.r#note == 21 { return 1123; }
        if self.r#instrument == Instrument::Snare && self.r#note == 13 && self.r#powered == false { return 708; }
        if self.r#note == 1 && self.r#instrument == Instrument::Dragon && self.r#powered == true { return 1533; }
        if self.r#instrument == Instrument::Flute && self.r#note == 22 && self.r#powered == true { return 875; }
        if self.r#powered == false && self.r#note == 22 && self.r#instrument == Instrument::Flute { return 876; }
        if self.r#instrument == Instrument::Chime && self.r#powered == true && self.r#note == 7 { return 995; }
        if self.r#powered == false && self.r#instrument == Instrument::Skeleton && self.r#note == 17 { return 1466; }
        if self.r#instrument == Instrument::Xylophone && self.r#note == 21 && self.r#powered == false { return 1074; }
        if self.r#note == 5 && self.r#powered == true && self.r#instrument == Instrument::Guitar { return 941; }
        if self.r#note == 19 && self.r#powered == false && self.r#instrument == Instrument::Guitar { return 970; }
        if self.r#instrument == Instrument::Skeleton && self.r#powered == false && self.r#note == 14 { return 1460; }
        if self.r#powered == true && self.r#instrument == Instrument::Creeper && self.r#note == 5 { return 1491; }
        if self.r#powered == true && self.r#note == 23 && self.r#instrument == Instrument::Banjo { return 1327; }
        if self.r#powered == false && self.r#note == 14 && self.r#instrument == Instrument::CustomHead { return 1710; }
        if self.r#note == 24 && self.r#powered == false && self.r#instrument == Instrument::Harp { return 630; }
        if self.r#note == 17 && self.r#powered == false && self.r#instrument == Instrument::IronXylophone { return 1116; }
        if self.r#powered == true && self.r#instrument == Instrument::Hat && self.r#note == 8 { return 747; }
        if self.r#note == 22 && self.r#instrument == Instrument::Chime && self.r#powered == true { return 1025; }
        if self.r#powered == false && self.r#note == 1 && self.r#instrument == Instrument::Chime { return 984; }
        if self.r#note == 24 && self.r#powered == false && self.r#instrument == Instrument::IronXylophone { return 1130; }
        if self.r#note == 18 && self.r#powered == false && self.r#instrument == Instrument::Zombie { return 1418; }
        if self.r#powered == true && self.r#instrument == Instrument::Zombie && self.r#note == 6 { return 1393; }
        if self.r#note == 9 && self.r#powered == true && self.r#instrument == Instrument::Piglin { return 1649; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 1 && self.r#powered == true { return 1283; }
        if self.r#note == 18 && self.r#instrument == Instrument::Guitar && self.r#powered == true { return 967; }
        if self.r#instrument == Instrument::Pling && self.r#powered == true && self.r#note == 10 { return 1351; }
        if self.r#note == 8 && self.r#instrument == Instrument::Dragon && self.r#powered == false { return 1548; }
        if self.r#instrument == Instrument::Zombie && self.r#note == 10 && self.r#powered == false { return 1402; }
        if self.r#powered == false && self.r#instrument == Instrument::Zombie && self.r#note == 4 { return 1390; }
        if self.r#instrument == Instrument::CowBell && self.r#note == 10 && self.r#powered == false { return 1152; }
        if self.r#powered == false && self.r#instrument == Instrument::Xylophone && self.r#note == 10 { return 1052; }
        if self.r#powered == true && self.r#instrument == Instrument::Pling && self.r#note == 6 { return 1343; }
        if self.r#powered == false && self.r#instrument == Instrument::Pling && self.r#note == 23 { return 1378; }
        if self.r#instrument == Instrument::Harp && self.r#note == 13 && self.r#powered == false { return 608; }
        if self.r#instrument == Instrument::Hat && self.r#powered == false && self.r#note == 0 { return 732; }
        if self.r#note == 12 && self.r#powered == false && self.r#instrument == Instrument::Bell { return 906; }
        if self.r#note == 13 && self.r#instrument == Instrument::Bit && self.r#powered == true { return 1257; }
        if self.r#instrument == Instrument::CowBell && self.r#note == 11 && self.r#powered == false { return 1154; }
        if self.r#note == 16 && self.r#instrument == Instrument::Pling && self.r#powered == false { return 1364; }
        if self.r#instrument == Instrument::Hat && self.r#powered == false && self.r#note == 16 { return 764; }
        if self.r#instrument == Instrument::Flute && self.r#note == 9 && self.r#powered == true { return 849; }
        if self.r#note == 12 && self.r#instrument == Instrument::Chime && self.r#powered == false { return 1006; }
        if self.r#instrument == Instrument::Pling && self.r#note == 19 && self.r#powered == false { return 1370; }
        if self.r#instrument == Instrument::Creeper && self.r#powered == true && self.r#note == 13 { return 1507; }
        if self.r#powered == false && self.r#instrument == Instrument::CowBell && self.r#note == 5 { return 1142; }
        if self.r#instrument == Instrument::CustomHead && self.r#powered == true && self.r#note == 0 { return 1681; }
        if self.r#instrument == Instrument::Skeleton && self.r#note == 13 && self.r#powered == true { return 1457; }
        if self.r#instrument == Instrument::Bit && self.r#note == 14 && self.r#powered == false { return 1260; }
        if self.r#instrument == Instrument::Dragon && self.r#powered == false && self.r#note == 11 { return 1554; }
        if self.r#note == 10 && self.r#instrument == Instrument::IronXylophone && self.r#powered == false { return 1102; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#powered == true && self.r#note == 15 { return 1611; }
        if self.r#instrument == Instrument::Skeleton && self.r#note == 3 && self.r#powered == true { return 1437; }
        if self.r#instrument == Instrument::Snare && self.r#note == 15 && self.r#powered == true { return 711; }
        if self.r#note == 16 && self.r#instrument == Instrument::Flute && self.r#powered == false { return 864; }
        if self.r#instrument == Instrument::Bit && self.r#powered == true && self.r#note == 15 { return 1261; }
        if self.r#powered == false && self.r#note == 15 && self.r#instrument == Instrument::Skeleton { return 1462; }
        if self.r#instrument == Instrument::Bit && self.r#note == 23 && self.r#powered == true { return 1277; }
        if self.r#note == 13 && self.r#powered == false && self.r#instrument == Instrument::Creeper { return 1508; }
        if self.r#instrument == Instrument::Piglin && self.r#powered == true && self.r#note == 10 { return 1651; }
        if self.r#powered == true && self.r#instrument == Instrument::Chime && self.r#note == 16 { return 1013; }
        if self.r#note == 17 && self.r#instrument == Instrument::Flute && self.r#powered == false { return 866; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#powered == true && self.r#note == 6 { return 1593; }
        if self.r#instrument == Instrument::Snare && self.r#powered == true && self.r#note == 10 { return 701; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 15 && self.r#powered == false { return 1512; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 17 && self.r#powered == false { return 1666; }
        if self.r#powered == false && self.r#instrument == Instrument::WitherSkeleton && self.r#note == 15 { return 1612; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 20 && self.r#powered == true { return 1721; }
        if self.r#instrument == Instrument::Hat && self.r#note == 13 && self.r#powered == false { return 758; }
        if self.r#note == 9 && self.r#instrument == Instrument::Dragon && self.r#powered == false { return 1550; }
        if self.r#note == 24 && self.r#instrument == Instrument::Flute && self.r#powered == false { return 880; }
        if self.r#instrument == Instrument::Harp && self.r#note == 24 && self.r#powered == true { return 629; }
        if self.r#powered == false && self.r#instrument == Instrument::Snare && self.r#note == 16 { return 714; }
        if self.r#note == 4 && self.r#instrument == Instrument::Basedrum && self.r#powered == true { return 639; }
        if self.r#instrument == Instrument::Chime && self.r#note == 15 && self.r#powered == false { return 1012; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 1 && self.r#powered == true { return 1633; }
        if self.r#powered == true && self.r#note == 7 && self.r#instrument == Instrument::CustomHead { return 1695; }
        if self.r#powered == false && self.r#instrument == Instrument::Hat && self.r#note == 7 { return 746; }
        if self.r#instrument == Instrument::Snare && self.r#note == 24 && self.r#powered == false { return 730; }
        if self.r#note == 11 && self.r#powered == false && self.r#instrument == Instrument::Piglin { return 1654; }
        if self.r#note == 14 && self.r#powered == false && self.r#instrument == Instrument::Piglin { return 1660; }
        if self.r#instrument == Instrument::Zombie && self.r#note == 5 && self.r#powered == false { return 1392; }
        if self.r#note == 19 && self.r#powered == false && self.r#instrument == Instrument::Chime { return 1020; }
        if self.r#instrument == Instrument::Bass && self.r#note == 19 && self.r#powered == true { return 819; }
        if self.r#powered == true && self.r#instrument == Instrument::Zombie && self.r#note == 3 { return 1387; }
        if self.r#note == 11 && self.r#powered == true && self.r#instrument == Instrument::Didgeridoo { return 1203; }
        if self.r#note == 3 && self.r#instrument == Instrument::Flute && self.r#powered == true { return 837; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 23 && self.r#powered == true { return 1527; }
        if self.r#powered == false && self.r#note == 12 && self.r#instrument == Instrument::Snare { return 706; }
        if self.r#instrument == Instrument::Harp && self.r#note == 18 && self.r#powered == true { return 617; }
        if self.r#note == 17 && self.r#powered == false && self.r#instrument == Instrument::Didgeridoo { return 1216; }
        if self.r#note == 21 && self.r#powered == true && self.r#instrument == Instrument::Didgeridoo { return 1223; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 5 && self.r#powered == true { return 1291; }
        if self.r#instrument == Instrument::Hat && self.r#note == 19 && self.r#powered == false { return 770; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 11 && self.r#powered == true { return 1103; }
        if self.r#powered == true && self.r#instrument == Instrument::Didgeridoo && self.r#note == 7 { return 1195; }
        if self.r#powered == true && self.r#instrument == Instrument::Banjo && self.r#note == 13 { return 1307; }
        if self.r#instrument == Instrument::Creeper && self.r#powered == true && self.r#note == 18 { return 1517; }
        if self.r#note == 9 && self.r#instrument == Instrument::Dragon && self.r#powered == true { return 1549; }
        if self.r#powered == true && self.r#note == 21 && self.r#instrument == Instrument::Guitar { return 973; }
        if self.r#note == 23 && self.r#instrument == Instrument::WitherSkeleton && self.r#powered == true { return 1627; }
        if self.r#powered == true && self.r#instrument == Instrument::CowBell && self.r#note == 2 { return 1135; }
        if self.r#instrument == Instrument::Piglin && self.r#powered == true && self.r#note == 17 { return 1665; }
        if self.r#instrument == Instrument::Xylophone && self.r#note == 8 && self.r#powered == false { return 1048; }
        if self.r#instrument == Instrument::IronXylophone && self.r#powered == false && self.r#note == 7 { return 1096; }
        if self.r#instrument == Instrument::IronXylophone && self.r#powered == true && self.r#note == 23 { return 1127; }
        if self.r#powered == true && self.r#instrument == Instrument::Xylophone && self.r#note == 1 { return 1033; }
        if self.r#note == 14 && self.r#powered == true && self.r#instrument == Instrument::Zombie { return 1409; }
        if self.r#powered == true && self.r#instrument == Instrument::Basedrum && self.r#note == 5 { return 641; }
        if self.r#powered == false && self.r#instrument == Instrument::Didgeridoo && self.r#note == 7 { return 1196; }
        if self.r#note == 23 && self.r#instrument == Instrument::Bit && self.r#powered == false { return 1278; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 1 && self.r#powered == false { return 1484; }
        if self.r#powered == true && self.r#instrument == Instrument::WitherSkeleton && self.r#note == 8 { return 1597; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 12 && self.r#powered == false { return 1306; }
        if self.r#instrument == Instrument::Dragon && self.r#note == 4 && self.r#powered == true { return 1539; }
        if self.r#powered == true && self.r#instrument == Instrument::Guitar && self.r#note == 20 { return 971; }
        if self.r#powered == false && self.r#note == 24 && self.r#instrument == Instrument::Bit { return 1280; }
        if self.r#note == 11 && self.r#instrument == Instrument::Dragon && self.r#powered == true { return 1553; }
        if self.r#powered == false && self.r#note == 15 && self.r#instrument == Instrument::Dragon { return 1562; }
        if self.r#powered == false && self.r#instrument == Instrument::Bell && self.r#note == 13 { return 908; }
        if self.r#note == 6 && self.r#powered == true && self.r#instrument == Instrument::Bass { return 793; }
        if self.r#instrument == Instrument::CowBell && self.r#note == 7 && self.r#powered == true { return 1145; }
        if self.r#note == 19 && self.r#instrument == Instrument::Harp && self.r#powered == true { return 619; }
        if self.r#note == 21 && self.r#instrument == Instrument::CowBell && self.r#powered == true { return 1173; }
        if self.r#powered == false && self.r#instrument == Instrument::Didgeridoo && self.r#note == 10 { return 1202; }
        if self.r#powered == true && self.r#note == 12 && self.r#instrument == Instrument::Banjo { return 1305; }
        if self.r#instrument == Instrument::Snare && self.r#note == 24 && self.r#powered == true { return 729; }
        if self.r#powered == false && self.r#instrument == Instrument::Flute && self.r#note == 12 { return 856; }
        if self.r#instrument == Instrument::Flute && self.r#note == 16 && self.r#powered == true { return 863; }
        if self.r#powered == false && self.r#instrument == Instrument::IronXylophone && self.r#note == 15 { return 1112; }
        if self.r#powered == false && self.r#instrument == Instrument::Flute && self.r#note == 4 { return 840; }
        if self.r#note == 8 && self.r#powered == true && self.r#instrument == Instrument::Creeper { return 1497; }
        if self.r#instrument == Instrument::CowBell && self.r#powered == true && self.r#note == 13 { return 1157; }
        if self.r#note == 2 && self.r#powered == true && self.r#instrument == Instrument::Banjo { return 1285; }
        if self.r#instrument == Instrument::Skeleton && self.r#powered == true && self.r#note == 6 { return 1443; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 5 && self.r#powered == false { return 642; }
        if self.r#instrument == Instrument::Pling && self.r#powered == true && self.r#note == 20 { return 1371; }
        if self.r#powered == false && self.r#note == 3 && self.r#instrument == Instrument::Skeleton { return 1438; }
        if self.r#powered == true && self.r#instrument == Instrument::Skeleton && self.r#note == 15 { return 1461; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#note == 8 && self.r#powered == false { return 1198; }
        if self.r#instrument == Instrument::Chime && self.r#note == 11 && self.r#powered == false { return 1004; }
        if self.r#instrument == Instrument::Chime && self.r#powered == false && self.r#note == 4 { return 990; }
        if self.r#note == 23 && self.r#instrument == Instrument::Bass && self.r#powered == false { return 828; }
        if self.r#instrument == Instrument::Bit && self.r#powered == false && self.r#note == 6 { return 1244; }
        if self.r#instrument == Instrument::Harp && self.r#note == 6 && self.r#powered == false { return 594; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 17 && self.r#powered == true { return 665; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 6 && self.r#powered == true { return 1093; }
        if self.r#powered == false && self.r#note == 6 && self.r#instrument == Instrument::WitherSkeleton { return 1594; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 23 && self.r#powered == true { return 1727; }
        if self.r#instrument == Instrument::Zombie && self.r#note == 15 && self.r#powered == false { return 1412; }
        if self.r#note == 4 && self.r#powered == true && self.r#instrument == Instrument::Flute { return 839; }
        if self.r#note == 5 && self.r#powered == true && self.r#instrument == Instrument::Bass { return 791; }
        if self.r#instrument == Instrument::Chime && self.r#powered == false && self.r#note == 18 { return 1018; }
        if self.r#instrument == Instrument::Hat && self.r#note == 11 && self.r#powered == true { return 753; }
        if self.r#note == 23 && self.r#instrument == Instrument::Flute && self.r#powered == true { return 877; }
        if self.r#instrument == Instrument::Skeleton && self.r#powered == true && self.r#note == 19 { return 1469; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 21 && self.r#powered == true { return 1523; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 4 && self.r#powered == true { return 1639; }
        if self.r#powered == false && self.r#instrument == Instrument::Bass && self.r#note == 6 { return 794; }
        if self.r#note == 17 && self.r#powered == true && self.r#instrument == Instrument::Didgeridoo { return 1215; }
        if self.r#powered == true && self.r#instrument == Instrument::Dragon && self.r#note == 10 { return 1551; }
        if self.r#note == 10 && self.r#powered == true && self.r#instrument == Instrument::CustomHead { return 1701; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 7 && self.r#powered == true { return 1095; }
        if self.r#instrument == Instrument::Bit && self.r#note == 0 && self.r#powered == false { return 1232; }
        if self.r#note == 11 && self.r#powered == false && self.r#instrument == Instrument::Bit { return 1254; }
        if self.r#powered == true && self.r#instrument == Instrument::WitherSkeleton && self.r#note == 22 { return 1625; }
        if self.r#powered == false && self.r#note == 1 && self.r#instrument == Instrument::Bit { return 1234; }
        if self.r#powered == false && self.r#instrument == Instrument::WitherSkeleton && self.r#note == 22 { return 1626; }
        if self.r#note == 5 && self.r#powered == true && self.r#instrument == Instrument::Bit { return 1241; }
        if self.r#instrument == Instrument::Flute && self.r#note == 8 && self.r#powered == true { return 847; }
        if self.r#powered == false && self.r#note == 21 && self.r#instrument == Instrument::CowBell { return 1174; }
        if self.r#powered == true && self.r#instrument == Instrument::Xylophone && self.r#note == 4 { return 1039; }
        if self.r#powered == false && self.r#instrument == Instrument::Bit && self.r#note == 9 { return 1250; }
        if self.r#note == 4 && self.r#instrument == Instrument::Dragon && self.r#powered == false { return 1540; }
        if self.r#instrument == Instrument::Flute && self.r#note == 19 && self.r#powered == false { return 870; }
        if self.r#instrument == Instrument::Xylophone && self.r#note == 5 && self.r#powered == false { return 1042; }
        if self.r#note == 11 && self.r#powered == true && self.r#instrument == Instrument::Bell { return 903; }
        if self.r#note == 15 && self.r#powered == true && self.r#instrument == Instrument::Didgeridoo { return 1211; }
        if self.r#instrument == Instrument::Bell && self.r#powered == false && self.r#note == 22 { return 926; }
        if self.r#instrument == Instrument::Bell && self.r#note == 15 && self.r#powered == true { return 911; }
        if self.r#powered == false && self.r#instrument == Instrument::Pling && self.r#note == 5 { return 1342; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 22 && self.r#powered == false { return 1726; }
        if self.r#instrument == Instrument::Snare && self.r#note == 6 && self.r#powered == false { return 694; }
        if self.r#note == 2 && self.r#instrument == Instrument::WitherSkeleton && self.r#powered == false { return 1586; }
        if self.r#note == 9 && self.r#powered == false && self.r#instrument == Instrument::WitherSkeleton { return 1600; }
        if self.r#note == 23 && self.r#instrument == Instrument::Dragon && self.r#powered == false { return 1578; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 22 && self.r#powered == false { return 676; }
        if self.r#note == 10 && self.r#instrument == Instrument::CowBell && self.r#powered == true { return 1151; }
        if self.r#instrument == Instrument::Harp && self.r#powered == true && self.r#note == 14 { return 609; }
        if self.r#note == 7 && self.r#instrument == Instrument::Chime && self.r#powered == false { return 996; }
        if self.r#instrument == Instrument::Flute && self.r#note == 14 && self.r#powered == false { return 860; }
        if self.r#note == 16 && self.r#powered == false && self.r#instrument == Instrument::Bell { return 914; }
        if self.r#note == 18 && self.r#powered == false && self.r#instrument == Instrument::Harp { return 618; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 21 && self.r#powered == true { return 1323; }
        if self.r#instrument == Instrument::Pling && self.r#powered == false && self.r#note == 4 { return 1340; }
        if self.r#note == 7 && self.r#powered == false && self.r#instrument == Instrument::Flute { return 846; }
        if self.r#powered == false && self.r#instrument == Instrument::Pling && self.r#note == 22 { return 1376; }
        if self.r#powered == false && self.r#instrument == Instrument::Dragon && self.r#note == 1 { return 1534; }
        if self.r#powered == false && self.r#instrument == Instrument::Pling && self.r#note == 20 { return 1372; }
        if self.r#instrument == Instrument::Harp && self.r#note == 20 && self.r#powered == false { return 622; }
        if self.r#instrument == Instrument::Xylophone && self.r#note == 16 && self.r#powered == true { return 1063; }
        if self.r#instrument == Instrument::Pling && self.r#powered == true && self.r#note == 14 { return 1359; }
        if self.r#powered == true && self.r#note == 1 && self.r#instrument == Instrument::Chime { return 983; }
        if self.r#powered == false && self.r#note == 10 && self.r#instrument == Instrument::Snare { return 702; }
        if self.r#note == 5 && self.r#powered == false && self.r#instrument == Instrument::Dragon { return 1542; }
        if self.r#note == 15 && self.r#powered == true && self.r#instrument == Instrument::Bass { return 811; }
        if self.r#note == 17 && self.r#powered == false && self.r#instrument == Instrument::Bell { return 916; }
        if self.r#instrument == Instrument::CowBell && self.r#powered == false && self.r#note == 16 { return 1164; }
        if self.r#note == 0 && self.r#powered == true && self.r#instrument == Instrument::IronXylophone { return 1081; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 16 && self.r#powered == false { return 1114; }
        if self.r#note == 24 && self.r#powered == true && self.r#instrument == Instrument::Banjo { return 1329; }
        if self.r#note == 3 && self.r#instrument == Instrument::Chime && self.r#powered == false { return 988; }
        if self.r#instrument == Instrument::Zombie && self.r#note == 17 && self.r#powered == true { return 1415; }
        if self.r#powered == true && self.r#instrument == Instrument::IronXylophone && self.r#note == 1 { return 1083; }
        if self.r#powered == false && self.r#instrument == Instrument::Bass && self.r#note == 2 { return 786; }
        if self.r#note == 4 && self.r#powered == true && self.r#instrument == Instrument::Pling { return 1339; }
        if self.r#instrument == Instrument::Xylophone && self.r#note == 19 && self.r#powered == false { return 1070; }
        if self.r#powered == false && self.r#instrument == Instrument::Snare && self.r#note == 20 { return 722; }
        if self.r#note == 5 && self.r#instrument == Instrument::Skeleton && self.r#powered == true { return 1441; }
        if self.r#powered == true && self.r#instrument == Instrument::Creeper && self.r#note == 24 { return 1529; }
        if self.r#powered == false && self.r#instrument == Instrument::Piglin && self.r#note == 9 { return 1650; }
        if self.r#powered == true && self.r#instrument == Instrument::Snare && self.r#note == 19 { return 719; }
        if self.r#powered == false && self.r#note == 1 && self.r#instrument == Instrument::Pling { return 1334; }
        if self.r#powered == true && self.r#note == 2 && self.r#instrument == Instrument::Zombie { return 1385; }
        if self.r#powered == false && self.r#note == 12 && self.r#instrument == Instrument::Skeleton { return 1456; }
        if self.r#note == 2 && self.r#powered == true && self.r#instrument == Instrument::Creeper { return 1485; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 18 && self.r#powered == true { return 1667; }
        if self.r#instrument == Instrument::Bit && self.r#powered == true && self.r#note == 8 { return 1247; }
        if self.r#instrument == Instrument::Bell && self.r#note == 5 && self.r#powered == true { return 891; }
        if self.r#note == 19 && self.r#powered == false && self.r#instrument == Instrument::Skeleton { return 1470; }
        if self.r#powered == true && self.r#instrument == Instrument::Bell && self.r#note == 6 { return 893; }
        if self.r#powered == true && self.r#instrument == Instrument::Bell && self.r#note == 12 { return 905; }
        if self.r#note == 22 && self.r#powered == true && self.r#instrument == Instrument::Skeleton { return 1475; }
        if self.r#powered == false && self.r#note == 2 && self.r#instrument == Instrument::Basedrum { return 636; }
        if self.r#note == 6 && self.r#instrument == Instrument::Snare && self.r#powered == true { return 693; }
        if self.r#powered == false && self.r#instrument == Instrument::Zombie && self.r#note == 23 { return 1428; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 16 && self.r#powered == true { return 1513; }
        if self.r#note == 2 && self.r#powered == true && self.r#instrument == Instrument::Chime { return 985; }
        if self.r#instrument == Instrument::Bass && self.r#powered == false && self.r#note == 13 { return 808; }
        if self.r#note == 8 && self.r#instrument == Instrument::CustomHead && self.r#powered == false { return 1698; }
        if self.r#note == 24 && self.r#instrument == Instrument::Hat && self.r#powered == false { return 780; }
        if self.r#note == 15 && self.r#powered == false && self.r#instrument == Instrument::CowBell { return 1162; }
        if self.r#instrument == Instrument::Hat && self.r#note == 14 && self.r#powered == true { return 759; }
        if self.r#note == 4 && self.r#instrument == Instrument::Bass && self.r#powered == true { return 789; }
        if self.r#note == 22 && self.r#instrument == Instrument::Bass && self.r#powered == false { return 826; }
        if self.r#powered == false && self.r#note == 18 && self.r#instrument == Instrument::Didgeridoo { return 1218; }
        if self.r#powered == false && self.r#note == 0 && self.r#instrument == Instrument::Piglin { return 1632; }
        if self.r#powered == false && self.r#instrument == Instrument::Guitar && self.r#note == 7 { return 946; }
        if self.r#powered == false && self.r#note == 8 && self.r#instrument == Instrument::Pling { return 1348; }
        if self.r#powered == false && self.r#instrument == Instrument::Snare && self.r#note == 23 { return 728; }
        if self.r#instrument == Instrument::Basedrum && self.r#powered == false && self.r#note == 24 { return 680; }
        if self.r#powered == false && self.r#instrument == Instrument::Harp && self.r#note == 4 { return 590; }
        if self.r#instrument == Instrument::Skeleton && self.r#note == 22 && self.r#powered == false { return 1476; }
        if self.r#powered == false && self.r#note == 6 && self.r#instrument == Instrument::Guitar { return 944; }
        if self.r#powered == false && self.r#note == 6 && self.r#instrument == Instrument::Hat { return 744; }
        if self.r#instrument == Instrument::Bell && self.r#powered == false && self.r#note == 10 { return 902; }
        if self.r#instrument == Instrument::CowBell && self.r#note == 19 && self.r#powered == false { return 1170; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 12 && self.r#powered == true { return 1655; }
        if self.r#powered == true && self.r#instrument == Instrument::Snare && self.r#note == 11 { return 703; }
        if self.r#powered == false && self.r#instrument == Instrument::Guitar && self.r#note == 4 { return 940; }
        if self.r#powered == true && self.r#note == 19 && self.r#instrument == Instrument::Dragon { return 1569; }
        if self.r#powered == true && self.r#note == 13 && self.r#instrument == Instrument::Harp { return 607; }
        if self.r#powered == false && self.r#instrument == Instrument::Piglin && self.r#note == 24 { return 1680; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 1 && self.r#powered == false { return 1684; }
        if self.r#instrument == Instrument::Pling && self.r#note == 14 && self.r#powered == false { return 1360; }
        if self.r#instrument == Instrument::Zombie && self.r#note == 16 && self.r#powered == true { return 1413; }
        if self.r#note == 19 && self.r#instrument == Instrument::Basedrum && self.r#powered == false { return 670; }
        if self.r#note == 5 && self.r#powered == true && self.r#instrument == Instrument::Flute { return 841; }
        if self.r#powered == true && self.r#instrument == Instrument::Chime && self.r#note == 11 { return 1003; }
        if self.r#powered == true && self.r#instrument == Instrument::CustomHead && self.r#note == 11 { return 1703; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 19 && self.r#powered == true { return 1519; }
        if self.r#instrument == Instrument::Chime && self.r#note == 20 && self.r#powered == false { return 1022; }
        if self.r#instrument == Instrument::Hat && self.r#powered == false && self.r#note == 5 { return 742; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 11 && self.r#powered == false { return 1304; }
        if self.r#powered == true && self.r#instrument == Instrument::Snare && self.r#note == 16 { return 713; }
        if self.r#note == 9 && self.r#powered == false && self.r#instrument == Instrument::Snare { return 700; }
        if self.r#instrument == Instrument::Harp && self.r#powered == true && self.r#note == 0 { return 581; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#powered == false && self.r#note == 3 { return 1588; }
        if self.r#note == 14 && self.r#powered == true && self.r#instrument == Instrument::Xylophone { return 1059; }
        if self.r#note == 18 && self.r#instrument == Instrument::IronXylophone && self.r#powered == true { return 1117; }
        if self.r#note == 3 && self.r#instrument == Instrument::Banjo && self.r#powered == true { return 1287; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 6 && self.r#powered == false { return 1294; }
        if self.r#powered == true && self.r#instrument == Instrument::Skeleton && self.r#note == 4 { return 1439; }
        if self.r#instrument == Instrument::Hat && self.r#note == 23 && self.r#powered == false { return 778; }
        if self.r#powered == true && self.r#note == 14 && self.r#instrument == Instrument::Didgeridoo { return 1209; }
        if self.r#note == 5 && self.r#powered == true && self.r#instrument == Instrument::Snare { return 691; }
        if self.r#instrument == Instrument::Bit && self.r#powered == true && self.r#note == 22 { return 1275; }
        if self.r#note == 14 && self.r#instrument == Instrument::WitherSkeleton && self.r#powered == false { return 1610; }
        if self.r#note == 4 && self.r#powered == false && self.r#instrument == Instrument::CustomHead { return 1690; }
        if self.r#instrument == Instrument::Skeleton && self.r#note == 0 && self.r#powered == true { return 1431; }
        if self.r#powered == false && self.r#note == 24 && self.r#instrument == Instrument::Dragon { return 1580; }
        if self.r#note == 22 && self.r#instrument == Instrument::Xylophone && self.r#powered == false { return 1076; }
        if self.r#note == 6 && self.r#powered == false && self.r#instrument == Instrument::Basedrum { return 644; }
        if self.r#instrument == Instrument::Hat && self.r#note == 18 && self.r#powered == true { return 767; }
        if self.r#instrument == Instrument::Harp && self.r#powered == true && self.r#note == 23 { return 627; }
        if self.r#powered == true && self.r#instrument == Instrument::Bass && self.r#note == 1 { return 783; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 22 && self.r#powered == true { return 1525; }
        if self.r#note == 8 && self.r#powered == false && self.r#instrument == Instrument::Hat { return 748; }
        if self.r#instrument == Instrument::Bass && self.r#powered == false && self.r#note == 0 { return 782; }
        if self.r#powered == false && self.r#instrument == Instrument::Skeleton && self.r#note == 6 { return 1444; }
        if self.r#powered == false && self.r#instrument == Instrument::CustomHead && self.r#note == 11 { return 1704; }
        if self.r#instrument == Instrument::Harp && self.r#note == 5 && self.r#powered == true { return 591; }
        if self.r#instrument == Instrument::CustomHead && self.r#powered == false && self.r#note == 15 { return 1712; }
        if self.r#powered == false && self.r#instrument == Instrument::Dragon && self.r#note == 2 { return 1536; }
        if self.r#powered == false && self.r#instrument == Instrument::Didgeridoo && self.r#note == 0 { return 1182; }
        if self.r#powered == false && self.r#note == 18 && self.r#instrument == Instrument::Skeleton { return 1468; }
        if self.r#powered == true && self.r#instrument == Instrument::Bass && self.r#note == 7 { return 795; }
        if self.r#note == 8 && self.r#instrument == Instrument::Harp && self.r#powered == true { return 597; }
        if self.r#powered == false && self.r#note == 4 && self.r#instrument == Instrument::Snare { return 690; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#powered == false && self.r#note == 24 { return 1630; }
        if self.r#powered == false && self.r#note == 2 && self.r#instrument == Instrument::Creeper { return 1486; }
        if self.r#note == 12 && self.r#instrument == Instrument::CowBell && self.r#powered == false { return 1156; }
        if self.r#note == 9 && self.r#powered == true && self.r#instrument == Instrument::Snare { return 699; }
        if self.r#powered == true && self.r#instrument == Instrument::Bass && self.r#note == 0 { return 781; }
        if self.r#note == 12 && self.r#powered == true && self.r#instrument == Instrument::Bit { return 1255; }
        if self.r#note == 20 && self.r#powered == false && self.r#instrument == Instrument::Guitar { return 972; }
        if self.r#note == 14 && self.r#instrument == Instrument::Basedrum && self.r#powered == true { return 659; }
        if self.r#note == 16 && self.r#instrument == Instrument::Hat && self.r#powered == true { return 763; }
        if self.r#instrument == Instrument::Dragon && self.r#note == 6 && self.r#powered == true { return 1543; }
        if self.r#powered == true && self.r#instrument == Instrument::Harp && self.r#note == 20 { return 621; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 24 && self.r#powered == false { return 1730; }
        if self.r#note == 9 && self.r#instrument == Instrument::Chime && self.r#powered == false { return 1000; }
        if self.r#note == 3 && self.r#powered == false && self.r#instrument == Instrument::Creeper { return 1488; }
        if self.r#powered == true && self.r#note == 21 && self.r#instrument == Instrument::Basedrum { return 673; }
        if self.r#instrument == Instrument::Xylophone && self.r#note == 13 && self.r#powered == true { return 1057; }
        if self.r#instrument == Instrument::Pling && self.r#note == 18 && self.r#powered == false { return 1368; }
        if self.r#powered == true && self.r#instrument == Instrument::Bass && self.r#note == 22 { return 825; }
        if self.r#powered == true && self.r#instrument == Instrument::Zombie && self.r#note == 13 { return 1407; }
        if self.r#note == 1 && self.r#instrument == Instrument::Xylophone && self.r#powered == false { return 1034; }
        if self.r#note == 0 && self.r#powered == true && self.r#instrument == Instrument::Piglin { return 1631; }
        if self.r#powered == false && self.r#note == 21 && self.r#instrument == Instrument::Didgeridoo { return 1224; }
        if self.r#powered == false && self.r#instrument == Instrument::Piglin && self.r#note == 12 { return 1656; }
        if self.r#instrument == Instrument::Snare && self.r#note == 2 && self.r#powered == true { return 685; }
        if self.r#note == 17 && self.r#instrument == Instrument::Guitar && self.r#powered == true { return 965; }
        if self.r#powered == false && self.r#instrument == Instrument::Didgeridoo && self.r#note == 20 { return 1222; }
        if self.r#instrument == Instrument::Dragon && self.r#powered == false && self.r#note == 14 { return 1560; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#note == 10 && self.r#powered == false { return 1602; }
        if self.r#powered == true && self.r#instrument == Instrument::Dragon && self.r#note == 21 { return 1573; }
        if self.r#powered == true && self.r#instrument == Instrument::Flute && self.r#note == 12 { return 855; }
        if self.r#instrument == Instrument::CowBell && self.r#note == 24 && self.r#powered == true { return 1179; }
        if self.r#powered == false && self.r#instrument == Instrument::Chime && self.r#note == 17 { return 1016; }
        if self.r#note == 15 && self.r#powered == true && self.r#instrument == Instrument::Pling { return 1361; }
        if self.r#instrument == Instrument::Dragon && self.r#note == 7 && self.r#powered == true { return 1545; }
        if self.r#note == 4 && self.r#instrument == Instrument::Hat && self.r#powered == false { return 740; }
        if self.r#note == 7 && self.r#instrument == Instrument::Basedrum && self.r#powered == true { return 645; }
        if self.r#note == 18 && self.r#powered == true && self.r#instrument == Instrument::Bell { return 917; }
        if self.r#instrument == Instrument::Bit && self.r#powered == true && self.r#note == 6 { return 1243; }
        if self.r#note == 10 && self.r#instrument == Instrument::Xylophone && self.r#powered == true { return 1051; }
        if self.r#powered == true && self.r#instrument == Instrument::Harp && self.r#note == 2 { return 585; }
        if self.r#powered == false && self.r#instrument == Instrument::Bit && self.r#note == 8 { return 1248; }
        if self.r#instrument == Instrument::Skeleton && self.r#note == 9 && self.r#powered == true { return 1449; }
        if self.r#instrument == Instrument::Zombie && self.r#note == 22 && self.r#powered == true { return 1425; }
        if self.r#note == 23 && self.r#instrument == Instrument::Skeleton && self.r#powered == false { return 1478; }
        if self.r#instrument == Instrument::Dragon && self.r#note == 12 && self.r#powered == false { return 1556; }
        if self.r#powered == false && self.r#instrument == Instrument::Dragon && self.r#note == 16 { return 1564; }
        if self.r#note == 7 && self.r#powered == true && self.r#instrument == Instrument::Hat { return 745; }
        if self.r#powered == false && self.r#instrument == Instrument::Bell && self.r#note == 20 { return 922; }
        if self.r#instrument == Instrument::Zombie && self.r#powered == true && self.r#note == 12 { return 1405; }
        if self.r#powered == false && self.r#instrument == Instrument::Skeleton && self.r#note == 7 { return 1446; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#note == 14 && self.r#powered == true { return 1609; }
        if self.r#instrument == Instrument::CowBell && self.r#note == 17 && self.r#powered == false { return 1166; }
        if self.r#powered == false && self.r#instrument == Instrument::Bit && self.r#note == 20 { return 1272; }
        if self.r#instrument == Instrument::Harp && self.r#note == 7 && self.r#powered == true { return 595; }
        if self.r#instrument == Instrument::Harp && self.r#note == 22 && self.r#powered == true { return 625; }
        if self.r#powered == false && self.r#instrument == Instrument::Chime && self.r#note == 0 { return 982; }
        if self.r#instrument == Instrument::Snare && self.r#note == 3 && self.r#powered == true { return 687; }
        if self.r#powered == true && self.r#instrument == Instrument::Xylophone && self.r#note == 6 { return 1043; }
        if self.r#powered == true && self.r#note == 10 && self.r#instrument == Instrument::Bit { return 1251; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 3 && self.r#powered == true { return 637; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#note == 12 && self.r#powered == true { return 1605; }
        if self.r#note == 15 && self.r#powered == true && self.r#instrument == Instrument::Chime { return 1011; }
        if self.r#note == 1 && self.r#instrument == Instrument::WitherSkeleton && self.r#powered == false { return 1584; }
        if self.r#powered == false && self.r#instrument == Instrument::Basedrum && self.r#note == 13 { return 658; }
        if self.r#instrument == Instrument::Flute && self.r#note == 24 && self.r#powered == true { return 879; }
        if self.r#powered == true && self.r#instrument == Instrument::IronXylophone && self.r#note == 20 { return 1121; }
        if self.r#note == 5 && self.r#powered == false && self.r#instrument == Instrument::Bit { return 1242; }
        if self.r#powered == false && self.r#note == 17 && self.r#instrument == Instrument::Creeper { return 1516; }
        if self.r#powered == true && self.r#note == 16 && self.r#instrument == Instrument::Piglin { return 1663; }
        if self.r#powered == true && self.r#instrument == Instrument::Banjo && self.r#note == 10 { return 1301; }
        if self.r#note == 2 && self.r#instrument == Instrument::Bit && self.r#powered == true { return 1235; }
        if self.r#instrument == Instrument::Guitar && self.r#note == 22 && self.r#powered == false { return 976; }
        if self.r#powered == false && self.r#note == 11 && self.r#instrument == Instrument::Zombie { return 1404; }
        if self.r#instrument == Instrument::Flute && self.r#note == 23 && self.r#powered == false { return 878; }
        if self.r#powered == true && self.r#note == 10 && self.r#instrument == Instrument::Creeper { return 1501; }
        if self.r#instrument == Instrument::Dragon && self.r#powered == true && self.r#note == 22 { return 1575; }
        if self.r#note == 3 && self.r#powered == false && self.r#instrument == Instrument::Piglin { return 1638; }
        if self.r#note == 0 && self.r#instrument == Instrument::Xylophone && self.r#powered == false { return 1032; }
        if self.r#powered == false && self.r#note == 3 && self.r#instrument == Instrument::Harp { return 588; }
        if self.r#instrument == Instrument::Skeleton && self.r#powered == false && self.r#note == 2 { return 1436; }
        if self.r#powered == false && self.r#note == 17 && self.r#instrument == Instrument::Snare { return 716; }
        if self.r#note == 21 && self.r#powered == false && self.r#instrument == Instrument::Bell { return 924; }
        if self.r#note == 12 && self.r#instrument == Instrument::IronXylophone && self.r#powered == false { return 1106; }
        if self.r#note == 8 && self.r#powered == false && self.r#instrument == Instrument::Zombie { return 1398; }
        if self.r#instrument == Instrument::CustomHead && self.r#powered == false && self.r#note == 3 { return 1688; }
        if self.r#powered == false && self.r#note == 14 && self.r#instrument == Instrument::Harp { return 610; }
        if self.r#note == 5 && self.r#instrument == Instrument::Piglin && self.r#powered == false { return 1642; }
        if self.r#note == 16 && self.r#instrument == Instrument::WitherSkeleton && self.r#powered == false { return 1614; }
        if self.r#powered == true && self.r#note == 8 && self.r#instrument == Instrument::Guitar { return 947; }
        if self.r#powered == true && self.r#instrument == Instrument::Skeleton && self.r#note == 21 { return 1473; }
        if self.r#powered == false && self.r#note == 1 && self.r#instrument == Instrument::Banjo { return 1284; }
        if self.r#powered == false && self.r#instrument == Instrument::Skeleton && self.r#note == 4 { return 1440; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 14 && self.r#powered == false { return 660; }
        if self.r#note == 9 && self.r#powered == true && self.r#instrument == Instrument::Didgeridoo { return 1199; }
        if self.r#instrument == Instrument::Flute && self.r#note == 2 && self.r#powered == true { return 835; }
        if self.r#note == 4 && self.r#powered == false && self.r#instrument == Instrument::Basedrum { return 640; }
        if self.r#powered == false && self.r#instrument == Instrument::Harp && self.r#note == 5 { return 592; }
        if self.r#note == 0 && self.r#powered == false && self.r#instrument == Instrument::Dragon { return 1532; }
        if self.r#powered == true && self.r#instrument == Instrument::IronXylophone && self.r#note == 3 { return 1087; }
        if self.r#note == 20 && self.r#powered == false && self.r#instrument == Instrument::WitherSkeleton { return 1622; }
        if self.r#note == 5 && self.r#powered == true && self.r#instrument == Instrument::Piglin { return 1641; }
        if self.r#note == 23 && self.r#powered == true && self.r#instrument == Instrument::Xylophone { return 1077; }
        if self.r#note == 4 && self.r#powered == true && self.r#instrument == Instrument::Harp { return 589; }
        if self.r#powered == false && self.r#instrument == Instrument::WitherSkeleton && self.r#note == 8 { return 1598; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 22 && self.r#powered == true { return 1125; }
        if self.r#instrument == Instrument::Bell && self.r#note == 6 && self.r#powered == false { return 894; }
        if self.r#powered == true && self.r#instrument == Instrument::Bell && self.r#note == 0 { return 881; }
        if self.r#powered == true && self.r#instrument == Instrument::Snare && self.r#note == 18 { return 717; }
        if self.r#instrument == Instrument::Pling && self.r#powered == true && self.r#note == 11 { return 1353; }
        if self.r#instrument == Instrument::CustomHead && self.r#powered == true && self.r#note == 22 { return 1725; }
        if self.r#powered == false && self.r#instrument == Instrument::IronXylophone && self.r#note == 0 { return 1082; }
        if self.r#note == 11 && self.r#powered == false && self.r#instrument == Instrument::Harp { return 604; }
        if self.r#powered == false && self.r#note == 24 && self.r#instrument == Instrument::Chime { return 1030; }
        if self.r#instrument == Instrument::Skeleton && self.r#note == 1 && self.r#powered == false { return 1434; }
        if self.r#instrument == Instrument::Hat && self.r#note == 23 && self.r#powered == true { return 777; }
        if self.r#note == 24 && self.r#instrument == Instrument::Bell && self.r#powered == true { return 929; }
        if self.r#powered == false && self.r#note == 19 && self.r#instrument == Instrument::Banjo { return 1320; }
        if self.r#note == 0 && self.r#instrument == Instrument::Hat && self.r#powered == true { return 731; }
        if self.r#powered == true && self.r#instrument == Instrument::Zombie && self.r#note == 8 { return 1397; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 23 && self.r#powered == false { return 1528; }
        if self.r#note == 17 && self.r#instrument == Instrument::CustomHead && self.r#powered == false { return 1716; }
        if self.r#powered == false && self.r#note == 0 && self.r#instrument == Instrument::Guitar { return 932; }
        if self.r#instrument == Instrument::Bell && self.r#powered == true && self.r#note == 13 { return 907; }
        if self.r#powered == false && self.r#instrument == Instrument::Guitar && self.r#note == 16 { return 964; }
        if self.r#note == 13 && self.r#powered == false && self.r#instrument == Instrument::CustomHead { return 1708; }
        if self.r#instrument == Instrument::Flute && self.r#note == 9 && self.r#powered == false { return 850; }
        if self.r#powered == true && self.r#note == 11 && self.r#instrument == Instrument::Bit { return 1253; }
        if self.r#instrument == Instrument::Hat && self.r#powered == false && self.r#note == 17 { return 766; }
        if self.r#note == 2 && self.r#instrument == Instrument::Didgeridoo && self.r#powered == true { return 1185; }
        if self.r#instrument == Instrument::Banjo && self.r#powered == false && self.r#note == 8 { return 1298; }
        if self.r#note == 12 && self.r#powered == true && self.r#instrument == Instrument::IronXylophone { return 1105; }
        if self.r#powered == false && self.r#note == 3 && self.r#instrument == Instrument::Hat { return 738; }
        if self.r#powered == true && self.r#instrument == Instrument::Banjo && self.r#note == 7 { return 1295; }
        if self.r#powered == true && self.r#note == 20 && self.r#instrument == Instrument::Banjo { return 1321; }
        if self.r#note == 17 && self.r#instrument == Instrument::Harp && self.r#powered == true { return 615; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#note == 10 && self.r#powered == true { return 1601; }
        if self.r#instrument == Instrument::Pling && self.r#note == 2 && self.r#powered == true { return 1335; }
        if self.r#instrument == Instrument::Skeleton && self.r#note == 2 && self.r#powered == true { return 1435; }
        if self.r#note == 1 && self.r#instrument == Instrument::Pling && self.r#powered == true { return 1333; }
        if self.r#note == 24 && self.r#powered == false && self.r#instrument == Instrument::Bass { return 830; }
        if self.r#powered == false && self.r#instrument == Instrument::Chime && self.r#note == 10 { return 1002; }
        if self.r#instrument == Instrument::CowBell && self.r#powered == true && self.r#note == 8 { return 1147; }
        if self.r#note == 22 && self.r#instrument == Instrument::Guitar && self.r#powered == true { return 975; }
        if self.r#powered == false && self.r#note == 1 && self.r#instrument == Instrument::Hat { return 734; }
        if self.r#powered == false && self.r#instrument == Instrument::Bass && self.r#note == 15 { return 812; }
        if self.r#note == 4 && self.r#instrument == Instrument::Bell && self.r#powered == true { return 889; }
        if self.r#instrument == Instrument::Hat && self.r#note == 12 && self.r#powered == true { return 755; }
        if self.r#powered == true && self.r#instrument == Instrument::Bell && self.r#note == 9 { return 899; }
        if self.r#note == 1 && self.r#powered == true && self.r#instrument == Instrument::Snare { return 683; }
        if self.r#note == 18 && self.r#powered == false && self.r#instrument == Instrument::Banjo { return 1318; }
        if self.r#powered == true && self.r#instrument == Instrument::Skeleton && self.r#note == 14 { return 1459; }
        if self.r#note == 14 && self.r#instrument == Instrument::Didgeridoo && self.r#powered == false { return 1210; }
        if self.r#powered == true && self.r#instrument == Instrument::Pling && self.r#note == 19 { return 1369; }
        if self.r#powered == true && self.r#instrument == Instrument::Harp && self.r#note == 21 { return 623; }
        if self.r#instrument == Instrument::Bass && self.r#powered == false && self.r#note == 21 { return 824; }
        if self.r#instrument == Instrument::Bell && self.r#note == 19 && self.r#powered == false { return 920; }
        if self.r#note == 7 && self.r#instrument == Instrument::Bit && self.r#powered == true { return 1245; }
        if self.r#powered == false && self.r#note == 15 && self.r#instrument == Instrument::Harp { return 612; }
        if self.r#instrument == Instrument::Snare && self.r#powered == true && self.r#note == 17 { return 715; }
        if self.r#instrument == Instrument::Flute && self.r#note == 11 && self.r#powered == true { return 853; }
        if self.r#powered == false && self.r#instrument == Instrument::Guitar && self.r#note == 3 { return 938; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 18 && self.r#powered == true { return 667; }
        if self.r#powered == true && self.r#note == 3 && self.r#instrument == Instrument::Dragon { return 1537; }
        if self.r#powered == true && self.r#note == 21 && self.r#instrument == Instrument::Snare { return 723; }
        if self.r#instrument == Instrument::Bell && self.r#powered == false && self.r#note == 8 { return 898; }
        if self.r#note == 19 && self.r#instrument == Instrument::Xylophone && self.r#powered == true { return 1069; }
        if self.r#powered == true && self.r#instrument == Instrument::CowBell && self.r#note == 11 { return 1153; }
        if self.r#powered == true && self.r#note == 3 && self.r#instrument == Instrument::Xylophone { return 1037; }
        if self.r#instrument == Instrument::CowBell && self.r#powered == true && self.r#note == 18 { return 1167; }
        if self.r#instrument == Instrument::Bass && self.r#powered == false && self.r#note == 4 { return 790; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#note == 6 && self.r#powered == false { return 1194; }
        if self.r#note == 20 && self.r#instrument == Instrument::Piglin && self.r#powered == false { return 1672; }
        if self.r#instrument == Instrument::Flute && self.r#note == 5 && self.r#powered == false { return 842; }
        if self.r#note == 3 && self.r#instrument == Instrument::CowBell && self.r#powered == true { return 1137; }
        if self.r#powered == true && self.r#note == 2 && self.r#instrument == Instrument::CustomHead { return 1685; }
        if self.r#instrument == Instrument::Banjo && self.r#powered == false && self.r#note == 7 { return 1296; }
        if self.r#instrument == Instrument::CustomHead && self.r#powered == true && self.r#note == 15 { return 1711; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 9 && self.r#powered == false { return 1300; }
        if self.r#note == 5 && self.r#instrument == Instrument::CowBell && self.r#powered == true { return 1141; }
        if self.r#instrument == Instrument::Skeleton && self.r#powered == false && self.r#note == 20 { return 1472; }
        if self.r#instrument == Instrument::Guitar && self.r#note == 7 && self.r#powered == true { return 945; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#powered == false && self.r#note == 5 { return 1592; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 19 && self.r#powered == true { return 1669; }
        if self.r#powered == true && self.r#note == 2 && self.r#instrument == Instrument::Guitar { return 935; }
        if self.r#note == 2 && self.r#instrument == Instrument::Bit && self.r#powered == false { return 1236; }
        if self.r#note == 11 && self.r#powered == true && self.r#instrument == Instrument::Skeleton { return 1453; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 5 && self.r#powered == true { return 1691; }
        if self.r#instrument == Instrument::Hat && self.r#note == 14 && self.r#powered == false { return 760; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 14 && self.r#powered == false { return 1510; }
        if self.r#instrument == Instrument::Skeleton && self.r#note == 20 && self.r#powered == true { return 1471; }
        if self.r#instrument == Instrument::Creeper && self.r#powered == true && self.r#note == 7 { return 1495; }
        if self.r#powered == false && self.r#instrument == Instrument::Piglin && self.r#note == 1 { return 1634; }
        if self.r#instrument == Instrument::Hat && self.r#powered == false && self.r#note == 10 { return 752; }
        if self.r#note == 9 && self.r#instrument == Instrument::Creeper && self.r#powered == false { return 1500; }
        if self.r#note == 11 && self.r#instrument == Instrument::Snare && self.r#powered == false { return 704; }
        if self.r#instrument == Instrument::Piglin && self.r#powered == true && self.r#note == 21 { return 1673; }
        if self.r#note == 19 && self.r#instrument == Instrument::Bass && self.r#powered == false { return 820; }
        if self.r#powered == false && self.r#instrument == Instrument::Basedrum && self.r#note == 0 { return 632; }
        if self.r#note == 9 && self.r#instrument == Instrument::CowBell && self.r#powered == false { return 1150; }
        if self.r#instrument == Instrument::Guitar && self.r#note == 13 && self.r#powered == false { return 958; }
        if self.r#powered == true && self.r#note == 22 && self.r#instrument == Instrument::Basedrum { return 675; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 13 && self.r#powered == true { return 657; }
        if self.r#note == 8 && self.r#powered == false && self.r#instrument == Instrument::Flute { return 848; }
        if self.r#powered == false && self.r#instrument == Instrument::Chime && self.r#note == 5 { return 992; }
        if self.r#note == 11 && self.r#powered == true && self.r#instrument == Instrument::WitherSkeleton { return 1603; }
        if self.r#powered == false && self.r#instrument == Instrument::Zombie && self.r#note == 6 { return 1394; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 6 && self.r#powered == false { return 1694; }
        if self.r#instrument == Instrument::Bit && self.r#note == 17 && self.r#powered == false { return 1266; }
        if self.r#note == 17 && self.r#powered == false && self.r#instrument == Instrument::Basedrum { return 666; }
        if self.r#instrument == Instrument::Basedrum && self.r#note == 16 && self.r#powered == false { return 664; }
        if self.r#instrument == Instrument::Bit && self.r#note == 16 && self.r#powered == true { return 1263; }
        if self.r#powered == false && self.r#instrument == Instrument::Piglin && self.r#note == 4 { return 1640; }
        if self.r#powered == false && self.r#note == 8 && self.r#instrument == Instrument::Bass { return 798; }
        if self.r#powered == true && self.r#instrument == Instrument::CustomHead && self.r#note == 14 { return 1709; }
        if self.r#powered == true && self.r#instrument == Instrument::Zombie && self.r#note == 23 { return 1427; }
        if self.r#instrument == Instrument::CowBell && self.r#powered == false && self.r#note == 24 { return 1180; }
        if self.r#powered == false && self.r#note == 22 && self.r#instrument == Instrument::Dragon { return 1576; }
        if self.r#note == 11 && self.r#instrument == Instrument::Bass && self.r#powered == true { return 803; }
        if self.r#powered == false && self.r#instrument == Instrument::Bit && self.r#note == 7 { return 1246; }
        if self.r#powered == false && self.r#note == 15 && self.r#instrument == Instrument::Guitar { return 962; }
        if self.r#instrument == Instrument::CustomHead && self.r#note == 13 && self.r#powered == true { return 1707; }
        if self.r#instrument == Instrument::Bell && self.r#note == 8 && self.r#powered == true { return 897; }
        if self.r#note == 9 && self.r#instrument == Instrument::Xylophone && self.r#powered == false { return 1050; }
        if self.r#instrument == Instrument::Bit && self.r#note == 15 && self.r#powered == false { return 1262; }
        if self.r#note == 18 && self.r#powered == false && self.r#instrument == Instrument::Basedrum { return 668; }
        if self.r#powered == false && self.r#note == 9 && self.r#instrument == Instrument::Hat { return 750; }
        if self.r#note == 7 && self.r#instrument == Instrument::Basedrum && self.r#powered == false { return 646; }
        if self.r#powered == false && self.r#note == 23 && self.r#instrument == Instrument::Chime { return 1028; }
        if self.r#note == 18 && self.r#powered == true && self.r#instrument == Instrument::Didgeridoo { return 1217; }
        if self.r#note == 13 && self.r#powered == true && self.r#instrument == Instrument::Piglin { return 1657; }
        if self.r#note == 19 && self.r#powered == false && self.r#instrument == Instrument::CustomHead { return 1720; }
        if self.r#instrument == Instrument::Flute && self.r#note == 7 && self.r#powered == true { return 845; }
        if self.r#powered == true && self.r#note == 7 && self.r#instrument == Instrument::Snare { return 695; }
        if self.r#instrument == Instrument::Hat && self.r#powered == true && self.r#note == 22 { return 775; }
        if self.r#note == 12 && self.r#instrument == Instrument::Xylophone && self.r#powered == false { return 1056; }
        if self.r#powered == true && self.r#instrument == Instrument::WitherSkeleton && self.r#note == 2 { return 1585; }
        if self.r#powered == false && self.r#instrument == Instrument::Harp && self.r#note == 2 { return 586; }
        if self.r#instrument == Instrument::Bass && self.r#note == 3 && self.r#powered == true { return 787; }
        if self.r#instrument == Instrument::Bass && self.r#note == 3 && self.r#powered == false { return 788; }
        if self.r#powered == true && self.r#instrument == Instrument::Chime && self.r#note == 12 { return 1005; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 23 && self.r#powered == true { return 1677; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 20 && self.r#powered == true { return 1671; }
        if self.r#powered == true && self.r#instrument == Instrument::Guitar && self.r#note == 24 { return 979; }
        if self.r#powered == false && self.r#instrument == Instrument::Bit && self.r#note == 13 { return 1258; }
        if self.r#instrument == Instrument::Hat && self.r#powered == true && self.r#note == 3 { return 737; }
        if self.r#powered == false && self.r#instrument == Instrument::Didgeridoo && self.r#note == 19 { return 1220; }
        if self.r#note == 19 && self.r#powered == false && self.r#instrument == Instrument::IronXylophone { return 1120; }
        if self.r#note == 18 && self.r#instrument == Instrument::Skeleton && self.r#powered == true { return 1467; }
        if self.r#powered == false && self.r#instrument == Instrument::Dragon && self.r#note == 20 { return 1572; }
        if self.r#powered == true && self.r#note == 22 && self.r#instrument == Instrument::CowBell { return 1175; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 6 && self.r#powered == true { return 1293; }
        if self.r#note == 19 && self.r#powered == true && self.r#instrument == Instrument::CustomHead { return 1719; }
        if self.r#instrument == Instrument::Guitar && self.r#note == 10 && self.r#powered == false { return 952; }
        if self.r#instrument == Instrument::Bell && self.r#note == 2 && self.r#powered == false { return 886; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 22 && self.r#powered == true { return 1325; }
        if self.r#powered == false && self.r#instrument == Instrument::Piglin && self.r#note == 10 { return 1652; }
        if self.r#instrument == Instrument::Guitar && self.r#note == 8 && self.r#powered == false { return 948; }
        if self.r#powered == true && self.r#instrument == Instrument::CowBell && self.r#note == 12 { return 1155; }
        if self.r#instrument == Instrument::Xylophone && self.r#note == 16 && self.r#powered == false { return 1064; }
        if self.r#note == 3 && self.r#powered == false && self.r#instrument == Instrument::Didgeridoo { return 1188; }
        if self.r#instrument == Instrument::Pling && self.r#note == 11 && self.r#powered == false { return 1354; }
        if self.r#note == 17 && self.r#instrument == Instrument::Creeper && self.r#powered == true { return 1515; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 8 && self.r#powered == true { return 1097; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 24 && self.r#powered == true { return 1679; }
        if self.r#powered == false && self.r#note == 3 && self.r#instrument == Instrument::Dragon { return 1538; }
        if self.r#powered == true && self.r#instrument == Instrument::Basedrum && self.r#note == 20 { return 671; }
        if self.r#note == 21 && self.r#powered == false && self.r#instrument == Instrument::Chime { return 1024; }
        if self.r#instrument == Instrument::Harp && self.r#note == 23 && self.r#powered == false { return 628; }
        if self.r#instrument == Instrument::Harp && self.r#note == 6 && self.r#powered == true { return 593; }
        if self.r#note == 3 && self.r#powered == true && self.r#instrument == Instrument::Chime { return 987; }
        if self.r#powered == true && self.r#instrument == Instrument::Chime && self.r#note == 5 { return 991; }
        if self.r#note == 12 && self.r#instrument == Instrument::Basedrum && self.r#powered == true { return 655; }
        if self.r#instrument == Instrument::Snare && self.r#note == 23 && self.r#powered == true { return 727; }
        if self.r#note == 13 && self.r#instrument == Instrument::Flute && self.r#powered == false { return 858; }
        if self.r#powered == true && self.r#instrument == Instrument::Flute && self.r#note == 14 { return 859; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#powered == true && self.r#note == 16 { return 1213; }
        if self.r#powered == false && self.r#instrument == Instrument::Flute && self.r#note == 11 { return 854; }
        if self.r#powered == false && self.r#note == 3 && self.r#instrument == Instrument::Bit { return 1238; }
        if self.r#note == 24 && self.r#powered == false && self.r#instrument == Instrument::Didgeridoo { return 1230; }
        if self.r#note == 24 && self.r#powered == true && self.r#instrument == Instrument::Pling { return 1379; }
        if self.r#note == 4 && self.r#powered == true && self.r#instrument == Instrument::Banjo { return 1289; }
        if self.r#powered == true && self.r#note == 19 && self.r#instrument == Instrument::Didgeridoo { return 1219; }
        if self.r#note == 8 && self.r#instrument == Instrument::Basedrum && self.r#powered == false { return 648; }
        if self.r#note == 4 && self.r#powered == true && self.r#instrument == Instrument::Guitar { return 939; }
        if self.r#instrument == Instrument::Guitar && self.r#powered == false && self.r#note == 18 { return 968; }
        if self.r#instrument == Instrument::Guitar && self.r#powered == false && self.r#note == 2 { return 936; }
        if self.r#note == 13 && self.r#powered == true && self.r#instrument == Instrument::WitherSkeleton { return 1607; }
        if self.r#instrument == Instrument::Bell && self.r#powered == true && self.r#note == 21 { return 923; }
        if self.r#instrument == Instrument::Chime && self.r#note == 16 && self.r#powered == false { return 1014; }
        if self.r#instrument == Instrument::Pling && self.r#powered == false && self.r#note == 12 { return 1356; }
        if self.r#note == 19 && self.r#instrument == Instrument::CowBell && self.r#powered == true { return 1169; }
        if self.r#note == 8 && self.r#powered == true && self.r#instrument == Instrument::Dragon { return 1547; }
        if self.r#powered == false && self.r#instrument == Instrument::CustomHead && self.r#note == 2 { return 1686; }
        if self.r#powered == true && self.r#instrument == Instrument::Harp && self.r#note == 10 { return 601; }
        if self.r#powered == false && self.r#instrument == Instrument::Piglin && self.r#note == 21 { return 1674; }
        if self.r#instrument == Instrument::Creeper && self.r#powered == false && self.r#note == 22 { return 1526; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 3 && self.r#powered == true { return 1487; }
        if self.r#powered == true && self.r#instrument == Instrument::Bit && self.r#note == 21 { return 1273; }
        if self.r#instrument == Instrument::Xylophone && self.r#powered == true && self.r#note == 8 { return 1047; }
        if self.r#powered == true && self.r#instrument == Instrument::Zombie && self.r#note == 20 { return 1421; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 16 && self.r#powered == false { return 1664; }
        if self.r#powered == true && self.r#note == 16 && self.r#instrument == Instrument::CustomHead { return 1713; }
        if self.r#note == 9 && self.r#powered == true && self.r#instrument == Instrument::IronXylophone { return 1099; }
        if self.r#powered == true && self.r#note == 2 && self.r#instrument == Instrument::Basedrum { return 635; }
        if self.r#instrument == Instrument::Guitar && self.r#powered == true && self.r#note == 23 { return 977; }
        if self.r#powered == false && self.r#instrument == Instrument::WitherSkeleton && self.r#note == 13 { return 1608; }
        if self.r#powered == true && self.r#instrument == Instrument::Bit && self.r#note == 14 { return 1259; }
        if self.r#note == 1 && self.r#powered == true && self.r#instrument == Instrument::Bit { return 1233; }
        if self.r#note == 23 && self.r#powered == false && self.r#instrument == Instrument::Banjo { return 1328; }
        if self.r#instrument == Instrument::Piglin && self.r#powered == false && self.r#note == 7 { return 1646; }
        if self.r#powered == true && self.r#note == 5 && self.r#instrument == Instrument::Zombie { return 1391; }
        if self.r#note == 12 && self.r#instrument == Instrument::Guitar && self.r#powered == false { return 956; }
        if self.r#note == 2 && self.r#powered == false && self.r#instrument == Instrument::Snare { return 686; }
        if self.r#powered == false && self.r#instrument == Instrument::Chime && self.r#note == 6 { return 994; }
        if self.r#note == 4 && self.r#powered == false && self.r#instrument == Instrument::Bit { return 1240; }
        if self.r#instrument == Instrument::CowBell && self.r#note == 4 && self.r#powered == false { return 1140; }
        if self.r#instrument == Instrument::Hat && self.r#powered == false && self.r#note == 2 { return 736; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 4 && self.r#powered == false { return 1090; }
        if self.r#instrument == Instrument::Piglin && self.r#note == 3 && self.r#powered == true { return 1637; }
        if self.r#instrument == Instrument::Pling && self.r#note == 0 && self.r#powered == false { return 1332; }
        if self.r#powered == false && self.r#note == 18 && self.r#instrument == Instrument::Creeper { return 1518; }
        if self.r#instrument == Instrument::Hat && self.r#powered == true && self.r#note == 15 { return 761; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#note == 23 && self.r#powered == true { return 1227; }
        if self.r#instrument == Instrument::Bit && self.r#note == 0 && self.r#powered == true { return 1231; }
        if self.r#note == 9 && self.r#powered == true && self.r#instrument == Instrument::Zombie { return 1399; }
        if self.r#instrument == Instrument::CustomHead && self.r#powered == false && self.r#note == 10 { return 1702; }
        if self.r#powered == false && self.r#instrument == Instrument::IronXylophone && self.r#note == 6 { return 1094; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 21 && self.r#powered == false { return 1324; }
        if self.r#note == 4 && self.r#powered == true && self.r#instrument == Instrument::CowBell { return 1139; }
        if self.r#note == 19 && self.r#powered == true && self.r#instrument == Instrument::Bit { return 1269; }
        if self.r#instrument == Instrument::Basedrum && self.r#powered == false && self.r#note == 15 { return 662; }
        if self.r#note == 21 && self.r#powered == false && self.r#instrument == Instrument::Guitar { return 974; }
        if self.r#note == 1 && self.r#powered == true && self.r#instrument == Instrument::CowBell { return 1133; }
        if self.r#instrument == Instrument::Creeper && self.r#powered == true && self.r#note == 4 { return 1489; }
        if self.r#note == 0 && self.r#powered == false && self.r#instrument == Instrument::Bell { return 882; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 13 && self.r#powered == false { return 1308; }
        if self.r#note == 18 && self.r#powered == false && self.r#instrument == Instrument::Dragon { return 1568; }
        if self.r#powered == false && self.r#instrument == Instrument::Skeleton && self.r#note == 16 { return 1464; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 20 && self.r#powered == true { return 1521; }
        if self.r#note == 20 && self.r#instrument == Instrument::Didgeridoo && self.r#powered == true { return 1221; }
        if self.r#note == 8 && self.r#instrument == Instrument::Banjo && self.r#powered == true { return 1297; }
        if self.r#note == 1 && self.r#instrument == Instrument::Basedrum && self.r#powered == true { return 633; }
        if self.r#powered == true && self.r#note == 13 && self.r#instrument == Instrument::Hat { return 757; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 5 && self.r#powered == true { return 1091; }
        if self.r#powered == false && self.r#instrument == Instrument::Skeleton && self.r#note == 10 { return 1452; }
        if self.r#note == 18 && self.r#powered == false && self.r#instrument == Instrument::CowBell { return 1168; }
        if self.r#instrument == Instrument::Snare && self.r#note == 21 && self.r#powered == false { return 724; }
        if self.r#powered == false && self.r#instrument == Instrument::Chime && self.r#note == 14 { return 1010; }
        if self.r#note == 4 && self.r#instrument == Instrument::Snare && self.r#powered == true { return 689; }
        if self.r#powered == true && self.r#instrument == Instrument::Creeper && self.r#note == 15 { return 1511; }
        if self.r#instrument == Instrument::Bass && self.r#note == 24 && self.r#powered == true { return 829; }
        if self.r#instrument == Instrument::CowBell && self.r#note == 22 && self.r#powered == false { return 1176; }
        if self.r#powered == true && self.r#note == 1 && self.r#instrument == Instrument::Skeleton { return 1433; }
        if self.r#instrument == Instrument::IronXylophone && self.r#note == 14 && self.r#powered == false { return 1110; }
        if self.r#note == 12 && self.r#powered == true && self.r#instrument == Instrument::Creeper { return 1505; }
        if self.r#powered == true && self.r#note == 20 && self.r#instrument == Instrument::CowBell { return 1171; }
        if self.r#instrument == Instrument::Harp && self.r#note == 15 && self.r#powered == true { return 611; }
        if self.r#instrument == Instrument::Flute && self.r#powered == false && self.r#note == 3 { return 838; }
        if self.r#instrument == Instrument::Guitar && self.r#note == 14 && self.r#powered == false { return 960; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#powered == false && self.r#note == 4 { return 1190; }
        if self.r#instrument == Instrument::Bass && self.r#note == 16 && self.r#powered == false { return 814; }
        if self.r#instrument == Instrument::Didgeridoo && self.r#powered == false && self.r#note == 13 { return 1208; }
        if self.r#instrument == Instrument::Bit && self.r#powered == false && self.r#note == 12 { return 1256; }
        if self.r#note == 7 && self.r#powered == true && self.r#instrument == Instrument::Skeleton { return 1445; }
        if self.r#note == 2 && self.r#powered == true && self.r#instrument == Instrument::IronXylophone { return 1085; }
        if self.r#instrument == Instrument::Bass && self.r#note == 20 && self.r#powered == true { return 821; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 6 && self.r#powered == false { return 1494; }
        if self.r#note == 2 && self.r#powered == true && self.r#instrument == Instrument::Dragon { return 1535; }
        if self.r#instrument == Instrument::Snare && self.r#note == 5 && self.r#powered == false { return 692; }
        if self.r#instrument == Instrument::Banjo && self.r#note == 9 && self.r#powered == true { return 1299; }
        if self.r#note == 11 && self.r#powered == true && self.r#instrument == Instrument::Piglin { return 1653; }
        if self.r#powered == true && self.r#instrument == Instrument::CustomHead && self.r#note == 8 { return 1697; }
        if self.r#note == 10 && self.r#instrument == Instrument::Bit && self.r#powered == false { return 1252; }
        if self.r#powered == true && self.r#instrument == Instrument::Zombie && self.r#note == 7 { return 1395; }
        if self.r#instrument == Instrument::Chime && self.r#note == 14 && self.r#powered == true { return 1009; }
        if self.r#powered == true && self.r#note == 9 && self.r#instrument == Instrument::CowBell { return 1149; }
        if self.r#instrument == Instrument::Dragon && self.r#note == 7 && self.r#powered == false { return 1546; }
        if self.r#instrument == Instrument::Guitar && self.r#note == 11 && self.r#powered == true { return 953; }
        if self.r#instrument == Instrument::Harp && self.r#powered == false && self.r#note == 22 { return 626; }
        if self.r#instrument == Instrument::Dragon && self.r#powered == false && self.r#note == 17 { return 1566; }
        if self.r#instrument == Instrument::Creeper && self.r#note == 9 && self.r#powered == true { return 1499; }
        if self.r#powered == true && self.r#instrument == Instrument::Flute && self.r#note == 0 { return 831; }
        if self.r#note == 15 && self.r#powered == false && self.r#instrument == Instrument::Xylophone { return 1062; }
        if self.r#note == 0 && self.r#powered == true && self.r#instrument == Instrument::CowBell { return 1131; }
        if self.r#powered == false && self.r#instrument == Instrument::Didgeridoo && self.r#note == 5 { return 1192; }
        if self.r#instrument == Instrument::Dragon && self.r#note == 19 && self.r#powered == false { return 1570; }
        if self.r#instrument == Instrument::Chime && self.r#note == 10 && self.r#powered == true { return 1001; }
        if self.r#instrument == Instrument::WitherSkeleton && self.r#note == 17 && self.r#powered == false { return 1616; }
        if self.r#powered == true && self.r#note == 14 && self.r#instrument == Instrument::Piglin { return 1659; }
        if self.r#note == 16 && self.r#powered == false && self.r#instrument == Instrument::Bit { return 1264; }
        if self.r#powered == true && self.r#instrument == Instrument::WitherSkeleton && self.r#note == 21 { return 1623; }
        if self.r#instrument == Instrument::Xylophone && self.r#powered == false && self.r#note == 2 { return 1036; }
        if self.r#note == 24 && self.r#instrument == Instrument::CustomHead && self.r#powered == true { return 1729; }
        if self.r#powered == true && self.r#instrument == Instrument::Didgeridoo && self.r#note == 4 { return 1189; }
        if self.r#instrument == Instrument::Snare && self.r#note == 8 && self.r#powered == false { return 698; }
        if self.r#instrument == Instrument::Xylophone && self.r#powered == true && self.r#note == 5 { return 1041; }
        if self.r#note == 17 && self.r#instrument == Instrument::Bell && self.r#powered == true { return 915; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1559 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#powered: true,
                r#note: 14,
            });
        }
        if state_id == 966 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#powered: false,
                r#note: 17,
            });
        }
        if state_id == 1416 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#note: 17,
                r#powered: false,
            });
        }
        if state_id == 989 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#powered: true,
                r#note: 4,
            });
        }
        if state_id == 999 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Chime,
                r#note: 9,
            });
        }
        if state_id == 1429 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 24,
                r#instrument: Instrument::Zombie,
            });
        }
        if state_id == 1699 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 9,
                r#powered: true,
            });
        }
        if state_id == 613 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 16,
                r#powered: true,
            });
        }
        if state_id == 1629 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 24,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 1683 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 1,
                r#powered: true,
            });
        }
        if state_id == 707 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Snare,
                r#note: 13,
            });
        }
        if state_id == 1225 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Didgeridoo,
                r#note: 22,
            });
        }
        if state_id == 1503 {
            return Some(NoteBlock {
                r#note: 11,
                r#instrument: Instrument::Creeper,
                r#powered: true,
            });
        }
        if state_id == 957 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#note: 13,
                r#powered: true,
            });
        }
        if state_id == 1026 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#note: 22,
                r#powered: false,
            });
        }
        if state_id == 1375 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 22,
                r#instrument: Instrument::Pling,
            });
        }
        if state_id == 1281 {
            return Some(NoteBlock {
                r#note: 0,
                r#powered: true,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 1509 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#powered: true,
                r#note: 14,
            });
        }
        if state_id == 1565 {
            return Some(NoteBlock {
                r#note: 17,
                r#powered: true,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 1589 {
            return Some(NoteBlock {
                r#note: 4,
                r#powered: true,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 805 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 12,
                r#powered: true,
            });
        }
        if state_id == 1061 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#note: 15,
                r#powered: true,
            });
        }
        if state_id == 1268 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bit,
                r#note: 18,
            });
        }
        if state_id == 1615 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::WitherSkeleton,
                r#note: 17,
            });
        }
        if state_id == 969 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#powered: true,
                r#note: 19,
            });
        }
        if state_id == 1465 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#note: 17,
                r#powered: true,
            });
        }
        if state_id == 1624 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 21,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 718 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Snare,
                r#note: 18,
            });
        }
        if state_id == 1053 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#note: 11,
                r#powered: true,
            });
        }
        if state_id == 1617 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#note: 18,
                r#powered: true,
            });
        }
        if state_id == 1705 {
            return Some(NoteBlock {
                r#note: 12,
                r#powered: true,
                r#instrument: Instrument::CustomHead,
            });
        }
        if state_id == 624 {
            return Some(NoteBlock {
                r#note: 21,
                r#powered: false,
                r#instrument: Instrument::Harp,
            });
        }
        if state_id == 1237 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#note: 3,
                r#powered: true,
            });
        }
        if state_id == 1723 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 21,
                r#powered: true,
            });
        }
        if state_id == 1662 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 15,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 1396 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#note: 7,
                r#powered: false,
            });
        }
        if state_id == 1492 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 5,
                r#powered: false,
            });
        }
        if state_id == 1577 {
            return Some(NoteBlock {
                r#note: 23,
                r#powered: true,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 1128 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 23,
                r#powered: false,
            });
        }
        if state_id == 823 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 21,
                r#instrument: Instrument::Bass,
            });
        }
        if state_id == 1451 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#powered: true,
                r#note: 10,
            });
        }
        if state_id == 981 {
            return Some(NoteBlock {
                r#note: 0,
                r#powered: true,
                r#instrument: Instrument::Chime,
            });
        }
        if state_id == 1479 {
            return Some(NoteBlock {
                r#note: 24,
                r#powered: true,
                r#instrument: Instrument::Skeleton,
            });
        }
        if state_id == 649 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 9,
                r#powered: true,
            });
        }
        if state_id == 1400 {
            return Some(NoteBlock {
                r#note: 9,
                r#instrument: Instrument::Zombie,
                r#powered: false,
            });
        }
        if state_id == 1124 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#powered: false,
                r#note: 21,
            });
        }
        if state_id == 1647 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 8,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 1678 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 23,
                r#powered: false,
            });
        }
        if state_id == 1682 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#powered: false,
                r#note: 0,
            });
        }
        if state_id == 1336 {
            return Some(NoteBlock {
                r#note: 2,
                r#powered: false,
                r#instrument: Instrument::Pling,
            });
        }
        if state_id == 797 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bass,
                r#note: 8,
            });
        }
        if state_id == 1019 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Chime,
                r#note: 19,
            });
        }
        if state_id == 1017 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Chime,
                r#note: 18,
            });
        }
        if state_id == 978 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#powered: false,
                r#note: 23,
            });
        }
        if state_id == 1313 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 16,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 1066 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#note: 17,
                r#powered: false,
            });
        }
        if state_id == 816 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bass,
                r#note: 17,
            });
        }
        if state_id == 643 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Basedrum,
                r#note: 6,
            });
        }
        if state_id == 1265 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#note: 17,
                r#powered: true,
            });
        }
        if state_id == 1419 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#powered: true,
                r#note: 19,
            });
        }
        if state_id == 799 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bass,
                r#note: 9,
            });
        }
        if state_id == 802 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 10,
                r#powered: false,
            });
        }
        if state_id == 1249 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#powered: true,
                r#note: 9,
            });
        }
        if state_id == 1092 {
            return Some(NoteBlock {
                r#note: 5,
                r#instrument: Instrument::IronXylophone,
                r#powered: false,
            });
        }
        if state_id == 1700 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#powered: false,
                r#note: 9,
            });
        }
        if state_id == 1288 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#powered: false,
                r#note: 3,
            });
        }
        if state_id == 596 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#powered: false,
                r#note: 7,
            });
        }
        if state_id == 1045 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Xylophone,
                r#note: 7,
            });
        }
        if state_id == 934 {
            return Some(NoteBlock {
                r#note: 1,
                r#powered: false,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 1424 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 21,
                r#instrument: Instrument::Zombie,
            });
        }
        if state_id == 1357 {
            return Some(NoteBlock {
                r#note: 13,
                r#powered: true,
                r#instrument: Instrument::Pling,
            });
        }
        if state_id == 1482 {
            return Some(NoteBlock {
                r#note: 0,
                r#instrument: Instrument::Creeper,
                r#powered: false,
            });
        }
        if state_id == 779 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 24,
                r#instrument: Instrument::Hat,
            });
        }
        if state_id == 771 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#powered: true,
                r#note: 20,
            });
        }
        if state_id == 1448 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#note: 8,
                r#powered: false,
            });
        }
        if state_id == 937 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Guitar,
                r#note: 3,
            });
        }
        if state_id == 1079 {
            return Some(NoteBlock {
                r#note: 24,
                r#powered: true,
                r#instrument: Instrument::Xylophone,
            });
        }
        if state_id == 1191 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#powered: true,
                r#note: 5,
            });
        }
        if state_id == 1365 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#note: 17,
                r#powered: true,
            });
        }
        if state_id == 1422 {
            return Some(NoteBlock {
                r#note: 20,
                r#powered: false,
                r#instrument: Instrument::Zombie,
            });
        }
        if state_id == 1374 {
            return Some(NoteBlock {
                r#note: 21,
                r#instrument: Instrument::Pling,
                r#powered: false,
            });
        }
        if state_id == 653 {
            return Some(NoteBlock {
                r#note: 11,
                r#powered: true,
                r#instrument: Instrument::Basedrum,
            });
        }
        if state_id == 1555 {
            return Some(NoteBlock {
                r#note: 12,
                r#powered: true,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 682 {
            return Some(NoteBlock {
                r#note: 0,
                r#powered: false,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 862 {
            return Some(NoteBlock {
                r#note: 15,
                r#powered: false,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 1490 {
            return Some(NoteBlock {
                r#note: 4,
                r#instrument: Instrument::Creeper,
                r#powered: false,
            });
        }
        if state_id == 1531 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Dragon,
                r#note: 0,
            });
        }
        if state_id == 869 {
            return Some(NoteBlock {
                r#note: 19,
                r#powered: true,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 1186 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#powered: false,
                r#note: 2,
            });
        }
        if state_id == 1310 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 14,
                r#powered: false,
            });
        }
        if state_id == 1160 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#powered: false,
                r#note: 14,
            });
        }
        if state_id == 963 {
            return Some(NoteBlock {
                r#note: 16,
                r#powered: true,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 1239 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#powered: true,
                r#note: 4,
            });
        }
        if state_id == 1658 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 13,
                r#powered: false,
            });
        }
        if state_id == 1693 {
            return Some(NoteBlock {
                r#note: 6,
                r#instrument: Instrument::CustomHead,
                r#powered: true,
            });
        }
        if state_id == 769 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 19,
                r#powered: true,
            });
        }
        if state_id == 800 {
            return Some(NoteBlock {
                r#note: 9,
                r#powered: false,
                r#instrument: Instrument::Bass,
            });
        }
        if state_id == 661 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Basedrum,
                r#note: 15,
            });
        }
        if state_id == 883 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 1,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 1506 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Creeper,
                r#note: 12,
            });
        }
        if state_id == 605 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 12,
                r#powered: true,
            });
        }
        if state_id == 1557 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#note: 13,
                r#powered: true,
            });
        }
        if state_id == 1270 {
            return Some(NoteBlock {
                r#note: 19,
                r#instrument: Instrument::Bit,
                r#powered: false,
            });
        }
        if state_id == 600 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#powered: false,
                r#note: 9,
            });
        }
        if state_id == 654 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#powered: false,
                r#note: 11,
            });
        }
        if state_id == 919 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#note: 19,
                r#powered: true,
            });
        }
        if state_id == 1015 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#powered: true,
                r#note: 17,
            });
        }
        if state_id == 1229 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#note: 24,
                r#powered: true,
            });
        }
        if state_id == 1403 {
            return Some(NoteBlock {
                r#note: 11,
                r#instrument: Instrument::Zombie,
                r#powered: true,
            });
        }
        if state_id == 765 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 17,
                r#instrument: Instrument::Hat,
            });
        }
        if state_id == 606 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 12,
                r#powered: false,
            });
        }
        if state_id == 980 {
            return Some(NoteBlock {
                r#note: 24,
                r#instrument: Instrument::Guitar,
                r#powered: false,
            });
        }
        if state_id == 1144 {
            return Some(NoteBlock {
                r#note: 6,
                r#powered: false,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 955 {
            return Some(NoteBlock {
                r#note: 12,
                r#powered: true,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 1184 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#note: 1,
                r#powered: false,
            });
        }
        if state_id == 1290 {
            return Some(NoteBlock {
                r#note: 4,
                r#instrument: Instrument::Banjo,
                r#powered: false,
            });
        }
        if state_id == 1317 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 18,
                r#powered: true,
            });
        }
        if state_id == 1349 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Pling,
                r#note: 9,
            });
        }
        if state_id == 1058 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Xylophone,
                r#note: 13,
            });
        }
        if state_id == 890 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bell,
                r#note: 4,
            });
        }
        if state_id == 1075 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 22,
                r#instrument: Instrument::Xylophone,
            });
        }
        if state_id == 1286 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 2,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 1406 {
            return Some(NoteBlock {
                r#note: 12,
                r#instrument: Instrument::Zombie,
                r#powered: false,
            });
        }
        if state_id == 743 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#powered: true,
                r#note: 6,
            });
        }
        if state_id == 1635 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 2,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 873 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 21,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 1067 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Xylophone,
                r#note: 18,
            });
        }
        if state_id == 868 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 18,
                r#powered: false,
            });
        }
        if state_id == 688 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 3,
                r#powered: false,
            });
        }
        if state_id == 1027 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#powered: true,
                r#note: 23,
            });
        }
        if state_id == 1040 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#powered: false,
                r#note: 4,
            });
        }
        if state_id == 1382 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Zombie,
                r#note: 0,
            });
        }
        if state_id == 997 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Chime,
                r#note: 8,
            });
        }
        if state_id == 1714 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::CustomHead,
                r#note: 16,
            });
        }
        if state_id == 1122 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#powered: false,
                r#note: 20,
            });
        }
        if state_id == 652 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 10,
                r#instrument: Instrument::Basedrum,
            });
        }
        if state_id == 843 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 6,
                r#powered: true,
            });
        }
        if state_id == 921 {
            return Some(NoteBlock {
                r#note: 20,
                r#powered: true,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 792 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 5,
                r#powered: false,
            });
        }
        if state_id == 1581 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#note: 0,
                r#powered: true,
            });
        }
        if state_id == 943 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Guitar,
                r#note: 6,
            });
        }
        if state_id == 909 {
            return Some(NoteBlock {
                r#note: 14,
                r#instrument: Instrument::Bell,
                r#powered: true,
            });
        }
        if state_id == 1100 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 9,
                r#powered: false,
            });
        }
        if state_id == 1068 {
            return Some(NoteBlock {
                r#note: 18,
                r#instrument: Instrument::Xylophone,
                r#powered: false,
            });
        }
        if state_id == 1599 {
            return Some(NoteBlock {
                r#note: 9,
                r#instrument: Instrument::WitherSkeleton,
                r#powered: true,
            });
        }
        if state_id == 1060 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 14,
                r#instrument: Instrument::Xylophone,
            });
        }
        if state_id == 910 {
            return Some(NoteBlock {
                r#note: 14,
                r#instrument: Instrument::Bell,
                r#powered: false,
            });
        }
        if state_id == 931 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#note: 0,
                r#powered: true,
            });
        }
        if state_id == 1143 {
            return Some(NoteBlock {
                r#note: 6,
                r#instrument: Instrument::CowBell,
                r#powered: true,
            });
        }
        if state_id == 1129 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::IronXylophone,
                r#note: 24,
            });
        }
        if state_id == 1355 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#note: 12,
                r#powered: true,
            });
        }
        if state_id == 616 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Harp,
                r#note: 17,
            });
        }
        if state_id == 892 {
            return Some(NoteBlock {
                r#note: 5,
                r#powered: false,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 1187 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#powered: true,
                r#note: 3,
            });
        }
        if state_id == 1706 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::CustomHead,
                r#note: 12,
            });
        }
        if state_id == 1644 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 6,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 705 {
            return Some(NoteBlock {
                r#note: 12,
                r#powered: true,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 1319 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 19,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 1303 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Banjo,
                r#note: 11,
            });
        }
        if state_id == 1561 {
            return Some(NoteBlock {
                r#note: 15,
                r#instrument: Instrument::Dragon,
                r#powered: true,
            });
        }
        if state_id == 1648 {
            return Some(NoteBlock {
                r#note: 8,
                r#instrument: Instrument::Piglin,
                r#powered: false,
            });
        }
        if state_id == 1496 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 7,
                r#instrument: Instrument::Creeper,
            });
        }
        if state_id == 1197 {
            return Some(NoteBlock {
                r#note: 8,
                r#powered: true,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 751 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 10,
                r#powered: true,
            });
        }
        if state_id == 1432 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Skeleton,
                r#note: 0,
            });
        }
        if state_id == 1477 {
            return Some(NoteBlock {
                r#note: 23,
                r#instrument: Instrument::Skeleton,
                r#powered: true,
            });
        }
        if state_id == 1108 {
            return Some(NoteBlock {
                r#note: 13,
                r#instrument: Instrument::IronXylophone,
                r#powered: false,
            });
        }
        if state_id == 1031 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#note: 0,
                r#powered: true,
            });
        }
        if state_id == 1201 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Didgeridoo,
                r#note: 10,
            });
        }
        if state_id == 1670 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Piglin,
                r#note: 19,
            });
        }
        if state_id == 1606 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#note: 12,
                r#powered: false,
            });
        }
        if state_id == 697 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Snare,
                r#note: 8,
            });
        }
        if state_id == 1587 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 3,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 1636 {
            return Some(NoteBlock {
                r#note: 2,
                r#powered: false,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 1111 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 15,
                r#instrument: Instrument::IronXylophone,
            });
        }
        if state_id == 1226 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Didgeridoo,
                r#note: 22,
            });
        }
        if state_id == 959 {
            return Some(NoteBlock {
                r#note: 14,
                r#powered: true,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 678 {
            return Some(NoteBlock {
                r#note: 23,
                r#powered: false,
                r#instrument: Instrument::Basedrum,
            });
        }
        if state_id == 735 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 2,
                r#instrument: Instrument::Hat,
            });
        }
        if state_id == 754 {
            return Some(NoteBlock {
                r#note: 11,
                r#instrument: Instrument::Hat,
                r#powered: false,
            });
        }
        if state_id == 1178 {
            return Some(NoteBlock {
                r#note: 23,
                r#powered: false,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 815 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#powered: true,
                r#note: 17,
            });
        }
        if state_id == 1341 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#note: 5,
                r#powered: true,
            });
        }
        if state_id == 785 {
            return Some(NoteBlock {
                r#note: 2,
                r#instrument: Instrument::Bass,
                r#powered: true,
            });
        }
        if state_id == 833 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 1,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 1384 {
            return Some(NoteBlock {
                r#note: 1,
                r#instrument: Instrument::Zombie,
                r#powered: false,
            });
        }
        if state_id == 885 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 2,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 867 {
            return Some(NoteBlock {
                r#note: 18,
                r#powered: true,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 954 {
            return Some(NoteBlock {
                r#note: 11,
                r#powered: false,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 1008 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#note: 13,
                r#powered: false,
            });
        }
        if state_id == 1086 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 2,
                r#powered: false,
            });
        }
        if state_id == 598 {
            return Some(NoteBlock {
                r#note: 8,
                r#powered: false,
                r#instrument: Instrument::Harp,
            });
        }
        if state_id == 1109 {
            return Some(NoteBlock {
                r#note: 14,
                r#instrument: Instrument::IronXylophone,
                r#powered: true,
            });
        }
        if state_id == 1326 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 22,
                r#powered: false,
            });
        }
        if state_id == 1530 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 24,
                r#powered: false,
            });
        }
        if state_id == 1367 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Pling,
                r#note: 18,
            });
        }
        if state_id == 1620 {
            return Some(NoteBlock {
                r#note: 19,
                r#powered: false,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 651 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 10,
                r#powered: true,
            });
        }
        if state_id == 1119 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 19,
                r#powered: true,
            });
        }
        if state_id == 1322 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 20,
                r#powered: false,
            });
        }
        if state_id == 865 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 17,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 832 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Flute,
                r#note: 0,
            });
        }
        if state_id == 896 {
            return Some(NoteBlock {
                r#note: 7,
                r#powered: false,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 1161 {
            return Some(NoteBlock {
                r#note: 15,
                r#powered: true,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 1377 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 23,
                r#instrument: Instrument::Pling,
            });
        }
        if state_id == 1098 {
            return Some(NoteBlock {
                r#note: 8,
                r#instrument: Instrument::IronXylophone,
                r#powered: false,
            });
        }
        if state_id == 1373 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#powered: true,
                r#note: 21,
            });
        }
        if state_id == 1692 {
            return Some(NoteBlock {
                r#note: 5,
                r#powered: false,
                r#instrument: Instrument::CustomHead,
            });
        }
        if state_id == 1571 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#note: 20,
                r#powered: true,
            });
        }
        if state_id == 634 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Basedrum,
                r#note: 1,
            });
        }
        if state_id == 1136 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#powered: false,
                r#note: 2,
            });
        }
        if state_id == 1442 {
            return Some(NoteBlock {
                r#note: 5,
                r#instrument: Instrument::Skeleton,
                r#powered: false,
            });
        }
        if state_id == 1483 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 1,
                r#powered: true,
            });
        }
        if state_id == 1689 {
            return Some(NoteBlock {
                r#note: 4,
                r#powered: true,
                r#instrument: Instrument::CustomHead,
            });
        }
        if state_id == 725 {
            return Some(NoteBlock {
                r#note: 22,
                r#instrument: Instrument::Snare,
                r#powered: true,
            });
        }
        if state_id == 776 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Hat,
                r#note: 22,
            });
        }
        if state_id == 1502 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Creeper,
                r#note: 10,
            });
        }
        if state_id == 1276 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 22,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 1071 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#note: 20,
                r#powered: true,
            });
        }
        if state_id == 1038 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Xylophone,
                r#note: 3,
            });
        }
        if state_id == 1411 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#note: 15,
                r#powered: true,
            });
        }
        if state_id == 1073 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Xylophone,
                r#note: 21,
            });
        }
        if state_id == 583 {
            return Some(NoteBlock {
                r#note: 1,
                r#powered: true,
                r#instrument: Instrument::Harp,
            });
        }
        if state_id == 1574 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#note: 21,
                r#powered: false,
            });
        }
        if state_id == 1115 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::IronXylophone,
                r#note: 17,
            });
        }
        if state_id == 1596 {
            return Some(NoteBlock {
                r#note: 7,
                r#powered: false,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 1126 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 22,
                r#instrument: Instrument::IronXylophone,
            });
        }
        if state_id == 1450 {
            return Some(NoteBlock {
                r#note: 9,
                r#instrument: Instrument::Skeleton,
                r#powered: false,
            });
        }
        if state_id == 772 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 20,
                r#powered: false,
            });
        }
        if state_id == 1101 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 10,
                r#powered: true,
            });
        }
        if state_id == 901 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#powered: true,
                r#note: 10,
            });
        }
        if state_id == 733 {
            return Some(NoteBlock {
                r#note: 1,
                r#powered: true,
                r#instrument: Instrument::Hat,
            });
        }
        if state_id == 1363 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#note: 16,
                r#powered: true,
            });
        }
        if state_id == 784 {
            return Some(NoteBlock {
                r#note: 1,
                r#powered: false,
                r#instrument: Instrument::Bass,
            });
        }
        if state_id == 1271 {
            return Some(NoteBlock {
                r#note: 20,
                r#powered: true,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 1023 {
            return Some(NoteBlock {
                r#note: 21,
                r#powered: true,
                r#instrument: Instrument::Chime,
            });
        }
        if state_id == 1035 {
            return Some(NoteBlock {
                r#note: 2,
                r#powered: true,
                r#instrument: Instrument::Xylophone,
            });
        }
        if state_id == 1049 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#powered: true,
                r#note: 9,
            });
        }
        if state_id == 1362 {
            return Some(NoteBlock {
                r#note: 15,
                r#instrument: Instrument::Pling,
                r#powered: false,
            });
        }
        if state_id == 650 {
            return Some(NoteBlock {
                r#note: 9,
                r#powered: false,
                r#instrument: Instrument::Basedrum,
            });
        }
        if state_id == 677 {
            return Some(NoteBlock {
                r#note: 23,
                r#powered: true,
                r#instrument: Instrument::Basedrum,
            });
        }
        if state_id == 1366 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Pling,
                r#note: 17,
            });
        }
        if state_id == 696 {
            return Some(NoteBlock {
                r#note: 7,
                r#instrument: Instrument::Snare,
                r#powered: false,
            });
        }
        if state_id == 1311 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Banjo,
                r#note: 15,
            });
        }
        if state_id == 1463 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Skeleton,
                r#note: 16,
            });
        }
        if state_id == 631 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#powered: true,
                r#note: 0,
            });
        }
        if state_id == 1722 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 20,
                r#powered: false,
            });
        }
        if state_id == 1520 {
            return Some(NoteBlock {
                r#note: 19,
                r#powered: false,
                r#instrument: Instrument::Creeper,
            });
        }
        if state_id == 827 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bass,
                r#note: 23,
            });
        }
        if state_id == 1302 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Banjo,
                r#note: 10,
            });
        }
        if state_id == 749 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Hat,
                r#note: 9,
            });
        }
        if state_id == 1065 {
            return Some(NoteBlock {
                r#note: 17,
                r#instrument: Instrument::Xylophone,
                r#powered: true,
            });
        }
        if state_id == 1021 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Chime,
                r#note: 20,
            });
        }
        if state_id == 1316 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Banjo,
                r#note: 17,
            });
        }
        if state_id == 1148 {
            return Some(NoteBlock {
                r#note: 8,
                r#powered: false,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 1579 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Dragon,
                r#note: 24,
            });
        }
        if state_id == 927 {
            return Some(NoteBlock {
                r#note: 23,
                r#powered: true,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 710 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Snare,
                r#note: 14,
            });
        }
        if state_id == 1358 {
            return Some(NoteBlock {
                r#note: 13,
                r#instrument: Instrument::Pling,
                r#powered: false,
            });
        }
        if state_id == 1138 {
            return Some(NoteBlock {
                r#note: 3,
                r#instrument: Instrument::CowBell,
                r#powered: false,
            });
        }
        if state_id == 1724 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 21,
                r#powered: false,
            });
        }
        if state_id == 1267 {
            return Some(NoteBlock {
                r#note: 18,
                r#instrument: Instrument::Bit,
                r#powered: true,
            });
        }
        if state_id == 1661 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 15,
                r#powered: true,
            });
        }
        if state_id == 684 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 1,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 1158 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::CowBell,
                r#note: 13,
            });
        }
        if state_id == 1522 {
            return Some(NoteBlock {
                r#note: 20,
                r#instrument: Instrument::Creeper,
                r#powered: false,
            });
        }
        if state_id == 773 {
            return Some(NoteBlock {
                r#note: 21,
                r#powered: true,
                r#instrument: Instrument::Hat,
            });
        }
        if state_id == 810 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bass,
                r#note: 14,
            });
        }
        if state_id == 1132 {
            return Some(NoteBlock {
                r#note: 0,
                r#powered: false,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 1514 {
            return Some(NoteBlock {
                r#note: 16,
                r#instrument: Instrument::Creeper,
                r#powered: false,
            });
        }
        if state_id == 599 {
            return Some(NoteBlock {
                r#note: 9,
                r#instrument: Instrument::Harp,
                r#powered: true,
            });
        }
        if state_id == 818 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 18,
                r#powered: false,
            });
        }
        if state_id == 1346 {
            return Some(NoteBlock {
                r#note: 7,
                r#powered: false,
                r#instrument: Instrument::Pling,
            });
        }
        if state_id == 913 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 16,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 1414 {
            return Some(NoteBlock {
                r#note: 16,
                r#powered: false,
                r#instrument: Instrument::Zombie,
            });
        }
        if state_id == 1541 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#note: 5,
                r#powered: true,
            });
        }
        if state_id == 647 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Basedrum,
                r#note: 8,
            });
        }
        if state_id == 852 {
            return Some(NoteBlock {
                r#note: 10,
                r#powered: false,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 961 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#note: 15,
                r#powered: true,
            });
        }
        if state_id == 1595 {
            return Some(NoteBlock {
                r#note: 7,
                r#powered: true,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 928 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bell,
                r#note: 23,
            });
        }
        if state_id == 1687 {
            return Some(NoteBlock {
                r#note: 3,
                r#instrument: Instrument::CustomHead,
                r#powered: true,
            });
        }
        if state_id == 801 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 10,
                r#powered: true,
            });
        }
        if state_id == 1089 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 4,
                r#powered: true,
            });
        }
        if state_id == 712 {
            return Some(NoteBlock {
                r#note: 15,
                r#powered: false,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 1044 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 6,
                r#instrument: Instrument::Xylophone,
            });
        }
        if state_id == 822 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 20,
                r#powered: false,
            });
        }
        if state_id == 1338 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#powered: false,
                r#note: 3,
            });
        }
        if state_id == 1619 {
            return Some(NoteBlock {
                r#note: 19,
                r#powered: true,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 1420 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Zombie,
                r#note: 19,
            });
        }
        if state_id == 739 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Hat,
                r#note: 4,
            });
        }
        if state_id == 1193 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 6,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 1200 {
            return Some(NoteBlock {
                r#note: 9,
                r#powered: false,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 584 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Harp,
                r#note: 1,
            });
        }
        if state_id == 1558 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Dragon,
                r#note: 13,
            });
        }
        if state_id == 844 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 6,
                r#powered: false,
            });
        }
        if state_id == 1643 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Piglin,
                r#note: 6,
            });
        }
        if state_id == 1621 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 20,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 1583 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#powered: true,
                r#note: 1,
            });
        }
        if state_id == 638 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#powered: false,
                r#note: 3,
            });
        }
        if state_id == 872 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Flute,
                r#note: 20,
            });
        }
        if state_id == 804 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bass,
                r#note: 11,
            });
        }
        if state_id == 1386 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#note: 2,
                r#powered: false,
            });
        }
        if state_id == 1628 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::WitherSkeleton,
                r#note: 23,
            });
        }
        if state_id == 986 {
            return Some(NoteBlock {
                r#note: 2,
                r#instrument: Instrument::Chime,
                r#powered: false,
            });
        }
        if state_id == 1380 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 24,
                r#instrument: Instrument::Pling,
            });
        }
        if state_id == 1181 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Didgeridoo,
                r#note: 0,
            });
        }
        if state_id == 1212 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#powered: false,
                r#note: 15,
            });
        }
        if state_id == 1417 {
            return Some(NoteBlock {
                r#note: 18,
                r#powered: true,
                r#instrument: Instrument::Zombie,
            });
        }
        if state_id == 1072 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 20,
                r#instrument: Instrument::Xylophone,
            });
        }
        if state_id == 1675 {
            return Some(NoteBlock {
                r#note: 22,
                r#instrument: Instrument::Piglin,
                r#powered: true,
            });
        }
        if state_id == 806 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 12,
                r#powered: false,
            });
        }
        if state_id == 1344 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#powered: false,
                r#note: 6,
            });
        }
        if state_id == 1604 {
            return Some(NoteBlock {
                r#note: 11,
                r#instrument: Instrument::WitherSkeleton,
                r#powered: false,
            });
        }
        if state_id == 887 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bell,
                r#note: 3,
            });
        }
        if state_id == 1159 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#note: 14,
                r#powered: true,
            });
        }
        if state_id == 1104 {
            return Some(NoteBlock {
                r#note: 11,
                r#powered: false,
                r#instrument: Instrument::IronXylophone,
            });
        }
        if state_id == 1676 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Piglin,
                r#note: 22,
            });
        }
        if state_id == 1054 {
            return Some(NoteBlock {
                r#note: 11,
                r#powered: false,
                r#instrument: Instrument::Xylophone,
            });
        }
        if state_id == 1055 {
            return Some(NoteBlock {
                r#note: 12,
                r#instrument: Instrument::Xylophone,
                r#powered: true,
            });
        }
        if state_id == 1107 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 13,
                r#powered: true,
            });
        }
        if state_id == 1183 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Didgeridoo,
                r#note: 1,
            });
        }
        if state_id == 1205 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#note: 12,
                r#powered: true,
            });
        }
        if state_id == 1504 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 11,
                r#powered: false,
            });
        }
        if state_id == 1331 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 0,
                r#instrument: Instrument::Pling,
            });
        }
        if state_id == 1177 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::CowBell,
                r#note: 23,
            });
        }
        if state_id == 1309 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Banjo,
                r#note: 14,
            });
        }
        if state_id == 1389 {
            return Some(NoteBlock {
                r#note: 4,
                r#instrument: Instrument::Zombie,
                r#powered: true,
            });
        }
        if state_id == 1007 {
            return Some(NoteBlock {
                r#note: 13,
                r#powered: true,
                r#instrument: Instrument::Chime,
            });
        }
        if state_id == 1591 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::WitherSkeleton,
                r#note: 5,
            });
        }
        if state_id == 813 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 16,
                r#powered: true,
            });
        }
        if state_id == 1582 {
            return Some(NoteBlock {
                r#note: 0,
                r#powered: false,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 582 {
            return Some(NoteBlock {
                r#note: 0,
                r#instrument: Instrument::Harp,
                r#powered: false,
            });
        }
        if state_id == 726 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 22,
                r#powered: false,
            });
        }
        if state_id == 1312 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Banjo,
                r#note: 15,
            });
        }
        if state_id == 904 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#note: 11,
                r#powered: false,
            });
        }
        if state_id == 602 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 10,
                r#powered: false,
            });
        }
        if state_id == 1088 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 3,
                r#powered: false,
            });
        }
        if state_id == 1454 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#note: 11,
                r#powered: false,
            });
        }
        if state_id == 1481 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#powered: true,
                r#note: 0,
            });
        }
        if state_id == 918 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bell,
                r#note: 18,
            });
        }
        if state_id == 1728 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 23,
                r#powered: false,
            });
        }
        if state_id == 1165 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#note: 17,
                r#powered: true,
            });
        }
        if state_id == 1590 {
            return Some(NoteBlock {
                r#note: 4,
                r#instrument: Instrument::WitherSkeleton,
                r#powered: false,
            });
        }
        if state_id == 1347 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Pling,
                r#note: 8,
            });
        }
        if state_id == 1718 {
            return Some(NoteBlock {
                r#note: 18,
                r#instrument: Instrument::CustomHead,
                r#powered: false,
            });
        }
        if state_id == 1563 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#note: 16,
                r#powered: true,
            });
        }
        if state_id == 950 {
            return Some(NoteBlock {
                r#note: 9,
                r#powered: false,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 1381 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Zombie,
                r#note: 0,
            });
        }
        if state_id == 1645 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#powered: true,
                r#note: 7,
            });
        }
        if state_id == 1352 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#note: 10,
                r#powered: false,
            });
        }
        if state_id == 762 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#powered: false,
                r#note: 15,
            });
        }
        if state_id == 1493 {
            return Some(NoteBlock {
                r#note: 6,
                r#powered: true,
                r#instrument: Instrument::Creeper,
            });
        }
        if state_id == 1292 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 5,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 884 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bell,
                r#note: 1,
            });
        }
        if state_id == 836 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Flute,
                r#note: 2,
            });
        }
        if state_id == 951 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#note: 10,
                r#powered: true,
            });
        }
        if state_id == 1314 {
            return Some(NoteBlock {
                r#note: 16,
                r#instrument: Instrument::Banjo,
                r#powered: false,
            });
        }
        if state_id == 1426 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Zombie,
                r#note: 22,
            });
        }
        if state_id == 1330 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Banjo,
                r#note: 24,
            });
        }
        if state_id == 1715 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 17,
                r#powered: true,
            });
        }
        if state_id == 1430 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#powered: false,
                r#note: 24,
            });
        }
        if state_id == 807 {
            return Some(NoteBlock {
                r#note: 13,
                r#powered: true,
                r#instrument: Instrument::Bass,
            });
        }
        if state_id == 900 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bell,
                r#note: 9,
            });
        }
        if state_id == 1279 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 24,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 1113 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 16,
                r#powered: true,
            });
        }
        if state_id == 669 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 19,
                r#powered: true,
            });
        }
        if state_id == 1228 {
            return Some(NoteBlock {
                r#note: 23,
                r#instrument: Instrument::Didgeridoo,
                r#powered: false,
            });
        }
        if state_id == 720 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 19,
                r#powered: false,
            });
        }
        if state_id == 1410 {
            return Some(NoteBlock {
                r#note: 14,
                r#instrument: Instrument::Zombie,
                r#powered: false,
            });
        }
        if state_id == 1455 {
            return Some(NoteBlock {
                r#note: 12,
                r#powered: true,
                r#instrument: Instrument::Skeleton,
            });
        }
        if state_id == 1567 {
            return Some(NoteBlock {
                r#note: 18,
                r#powered: true,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 681 {
            return Some(NoteBlock {
                r#note: 0,
                r#instrument: Instrument::Snare,
                r#powered: true,
            });
        }
        if state_id == 942 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Guitar,
                r#note: 5,
            });
        }
        if state_id == 1282 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Banjo,
                r#note: 0,
            });
        }
        if state_id == 1388 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#powered: false,
                r#note: 3,
            });
        }
        if state_id == 925 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#powered: true,
                r#note: 22,
            });
        }
        if state_id == 756 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Hat,
                r#note: 12,
            });
        }
        if state_id == 656 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 12,
                r#powered: false,
            });
        }
        if state_id == 1618 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#note: 18,
                r#powered: false,
            });
        }
        if state_id == 587 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 3,
                r#instrument: Instrument::Harp,
            });
        }
        if state_id == 774 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 21,
                r#powered: false,
            });
        }
        if state_id == 1408 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#note: 13,
                r#powered: false,
            });
        }
        if state_id == 674 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 21,
                r#powered: false,
            });
        }
        if state_id == 949 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#powered: true,
                r#note: 9,
            });
        }
        if state_id == 663 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Basedrum,
                r#note: 16,
            });
        }
        if state_id == 895 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#powered: true,
                r#note: 7,
            });
        }
        if state_id == 998 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Chime,
                r#note: 8,
            });
        }
        if state_id == 857 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 13,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 1717 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 18,
                r#powered: true,
            });
        }
        if state_id == 1544 {
            return Some(NoteBlock {
                r#note: 6,
                r#powered: false,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 1552 {
            return Some(NoteBlock {
                r#note: 10,
                r#powered: false,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 679 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 24,
                r#powered: true,
            });
        }
        if state_id == 1696 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#powered: false,
                r#note: 7,
            });
        }
        if state_id == 1172 {
            return Some(NoteBlock {
                r#note: 20,
                r#instrument: Instrument::CowBell,
                r#powered: false,
            });
        }
        if state_id == 1668 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 18,
                r#powered: false,
            });
        }
        if state_id == 614 {
            return Some(NoteBlock {
                r#note: 16,
                r#instrument: Instrument::Harp,
                r#powered: false,
            });
        }
        if state_id == 851 {
            return Some(NoteBlock {
                r#note: 10,
                r#instrument: Instrument::Flute,
                r#powered: true,
            });
        }
        if state_id == 1474 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#note: 21,
                r#powered: false,
            });
        }
        if state_id == 809 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bass,
                r#note: 14,
            });
        }
        if state_id == 1524 {
            return Some(NoteBlock {
                r#note: 21,
                r#powered: false,
                r#instrument: Instrument::Creeper,
            });
        }
        if state_id == 912 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bell,
                r#note: 15,
            });
        }
        if state_id == 1118 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 18,
                r#powered: false,
            });
        }
        if state_id == 888 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#note: 3,
                r#powered: false,
            });
        }
        if state_id == 933 {
            return Some(NoteBlock {
                r#note: 1,
                r#powered: true,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 1315 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 17,
                r#powered: true,
            });
        }
        if state_id == 861 {
            return Some(NoteBlock {
                r#note: 15,
                r#instrument: Instrument::Flute,
                r#powered: true,
            });
        }
        if state_id == 817 {
            return Some(NoteBlock {
                r#note: 18,
                r#powered: true,
                r#instrument: Instrument::Bass,
            });
        }
        if state_id == 1613 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::WitherSkeleton,
                r#note: 16,
            });
        }
        if state_id == 1498 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Creeper,
                r#note: 8,
            });
        }
        if state_id == 1458 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#powered: false,
                r#note: 13,
            });
        }
        if state_id == 871 {
            return Some(NoteBlock {
                r#note: 20,
                r#powered: true,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 1214 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#note: 16,
                r#powered: false,
            });
        }
        if state_id == 709 {
            return Some(NoteBlock {
                r#note: 14,
                r#instrument: Instrument::Snare,
                r#powered: true,
            });
        }
        if state_id == 768 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 18,
                r#powered: false,
            });
        }
        if state_id == 1134 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#powered: false,
                r#note: 1,
            });
        }
        if state_id == 1204 {
            return Some(NoteBlock {
                r#note: 11,
                r#powered: false,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 1274 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bit,
                r#note: 21,
            });
        }
        if state_id == 1207 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 13,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 721 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Snare,
                r#note: 20,
            });
        }
        if state_id == 1163 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#note: 16,
                r#powered: true,
            });
        }
        if state_id == 1206 {
            return Some(NoteBlock {
                r#note: 12,
                r#instrument: Instrument::Didgeridoo,
                r#powered: false,
            });
        }
        if state_id == 1345 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Pling,
                r#note: 7,
            });
        }
        if state_id == 993 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#note: 6,
                r#powered: true,
            });
        }
        if state_id == 1447 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Skeleton,
                r#note: 8,
            });
        }
        if state_id == 603 {
            return Some(NoteBlock {
                r#note: 11,
                r#instrument: Instrument::Harp,
                r#powered: true,
            });
        }
        if state_id == 672 {
            return Some(NoteBlock {
                r#note: 20,
                r#instrument: Instrument::Basedrum,
                r#powered: false,
            });
        }
        if state_id == 1146 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::CowBell,
                r#note: 7,
            });
        }
        if state_id == 1084 {
            return Some(NoteBlock {
                r#note: 1,
                r#powered: false,
                r#instrument: Instrument::IronXylophone,
            });
        }
        if state_id == 1383 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#note: 1,
                r#powered: true,
            });
        }
        if state_id == 834 {
            return Some(NoteBlock {
                r#note: 1,
                r#instrument: Instrument::Flute,
                r#powered: false,
            });
        }
        if state_id == 930 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bell,
                r#note: 24,
            });
        }
        if state_id == 1080 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Xylophone,
                r#note: 24,
            });
        }
        if state_id == 1423 {
            return Some(NoteBlock {
                r#note: 21,
                r#instrument: Instrument::Zombie,
                r#powered: true,
            });
        }
        if state_id == 1480 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#powered: false,
                r#note: 24,
            });
        }
        if state_id == 1046 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Xylophone,
                r#note: 7,
            });
        }
        if state_id == 796 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bass,
                r#note: 7,
            });
        }
        if state_id == 1401 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#note: 10,
                r#powered: true,
            });
        }
        if state_id == 1029 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#powered: true,
                r#note: 24,
            });
        }
        if state_id == 620 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 19,
                r#instrument: Instrument::Harp,
            });
        }
        if state_id == 1350 {
            return Some(NoteBlock {
                r#note: 9,
                r#instrument: Instrument::Pling,
                r#powered: false,
            });
        }
        if state_id == 741 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#powered: true,
                r#note: 5,
            });
        }
        if state_id == 1078 {
            return Some(NoteBlock {
                r#note: 23,
                r#instrument: Instrument::Xylophone,
                r#powered: false,
            });
        }
        if state_id == 1337 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Pling,
                r#note: 3,
            });
        }
        if state_id == 874 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Flute,
                r#note: 21,
            });
        }
        if state_id == 1123 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#powered: true,
                r#note: 21,
            });
        }
        if state_id == 708 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 13,
                r#powered: false,
            });
        }
        if state_id == 1533 {
            return Some(NoteBlock {
                r#note: 1,
                r#instrument: Instrument::Dragon,
                r#powered: true,
            });
        }
        if state_id == 875 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 22,
                r#powered: true,
            });
        }
        if state_id == 876 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 22,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 995 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#powered: true,
                r#note: 7,
            });
        }
        if state_id == 1466 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Skeleton,
                r#note: 17,
            });
        }
        if state_id == 1074 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#note: 21,
                r#powered: false,
            });
        }
        if state_id == 941 {
            return Some(NoteBlock {
                r#note: 5,
                r#powered: true,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 970 {
            return Some(NoteBlock {
                r#note: 19,
                r#powered: false,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 1460 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#powered: false,
                r#note: 14,
            });
        }
        if state_id == 1491 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Creeper,
                r#note: 5,
            });
        }
        if state_id == 1327 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 23,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 1710 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 14,
                r#instrument: Instrument::CustomHead,
            });
        }
        if state_id == 630 {
            return Some(NoteBlock {
                r#note: 24,
                r#powered: false,
                r#instrument: Instrument::Harp,
            });
        }
        if state_id == 1116 {
            return Some(NoteBlock {
                r#note: 17,
                r#powered: false,
                r#instrument: Instrument::IronXylophone,
            });
        }
        if state_id == 747 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Hat,
                r#note: 8,
            });
        }
        if state_id == 1025 {
            return Some(NoteBlock {
                r#note: 22,
                r#instrument: Instrument::Chime,
                r#powered: true,
            });
        }
        if state_id == 984 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 1,
                r#instrument: Instrument::Chime,
            });
        }
        if state_id == 1130 {
            return Some(NoteBlock {
                r#note: 24,
                r#powered: false,
                r#instrument: Instrument::IronXylophone,
            });
        }
        if state_id == 1418 {
            return Some(NoteBlock {
                r#note: 18,
                r#powered: false,
                r#instrument: Instrument::Zombie,
            });
        }
        if state_id == 1393 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Zombie,
                r#note: 6,
            });
        }
        if state_id == 1649 {
            return Some(NoteBlock {
                r#note: 9,
                r#powered: true,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 1283 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 1,
                r#powered: true,
            });
        }
        if state_id == 967 {
            return Some(NoteBlock {
                r#note: 18,
                r#instrument: Instrument::Guitar,
                r#powered: true,
            });
        }
        if state_id == 1351 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#powered: true,
                r#note: 10,
            });
        }
        if state_id == 1548 {
            return Some(NoteBlock {
                r#note: 8,
                r#instrument: Instrument::Dragon,
                r#powered: false,
            });
        }
        if state_id == 1402 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#note: 10,
                r#powered: false,
            });
        }
        if state_id == 1390 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Zombie,
                r#note: 4,
            });
        }
        if state_id == 1152 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#note: 10,
                r#powered: false,
            });
        }
        if state_id == 1052 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Xylophone,
                r#note: 10,
            });
        }
        if state_id == 1343 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Pling,
                r#note: 6,
            });
        }
        if state_id == 1378 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Pling,
                r#note: 23,
            });
        }
        if state_id == 608 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 13,
                r#powered: false,
            });
        }
        if state_id == 732 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#powered: false,
                r#note: 0,
            });
        }
        if state_id == 906 {
            return Some(NoteBlock {
                r#note: 12,
                r#powered: false,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 1257 {
            return Some(NoteBlock {
                r#note: 13,
                r#instrument: Instrument::Bit,
                r#powered: true,
            });
        }
        if state_id == 1154 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#note: 11,
                r#powered: false,
            });
        }
        if state_id == 1364 {
            return Some(NoteBlock {
                r#note: 16,
                r#instrument: Instrument::Pling,
                r#powered: false,
            });
        }
        if state_id == 764 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#powered: false,
                r#note: 16,
            });
        }
        if state_id == 849 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 9,
                r#powered: true,
            });
        }
        if state_id == 1006 {
            return Some(NoteBlock {
                r#note: 12,
                r#instrument: Instrument::Chime,
                r#powered: false,
            });
        }
        if state_id == 1370 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#note: 19,
                r#powered: false,
            });
        }
        if state_id == 1507 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#powered: true,
                r#note: 13,
            });
        }
        if state_id == 1142 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::CowBell,
                r#note: 5,
            });
        }
        if state_id == 1681 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#powered: true,
                r#note: 0,
            });
        }
        if state_id == 1457 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#note: 13,
                r#powered: true,
            });
        }
        if state_id == 1260 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#note: 14,
                r#powered: false,
            });
        }
        if state_id == 1554 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#powered: false,
                r#note: 11,
            });
        }
        if state_id == 1102 {
            return Some(NoteBlock {
                r#note: 10,
                r#instrument: Instrument::IronXylophone,
                r#powered: false,
            });
        }
        if state_id == 1611 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#powered: true,
                r#note: 15,
            });
        }
        if state_id == 1437 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#note: 3,
                r#powered: true,
            });
        }
        if state_id == 711 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 15,
                r#powered: true,
            });
        }
        if state_id == 864 {
            return Some(NoteBlock {
                r#note: 16,
                r#instrument: Instrument::Flute,
                r#powered: false,
            });
        }
        if state_id == 1261 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#powered: true,
                r#note: 15,
            });
        }
        if state_id == 1462 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 15,
                r#instrument: Instrument::Skeleton,
            });
        }
        if state_id == 1277 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#note: 23,
                r#powered: true,
            });
        }
        if state_id == 1508 {
            return Some(NoteBlock {
                r#note: 13,
                r#powered: false,
                r#instrument: Instrument::Creeper,
            });
        }
        if state_id == 1651 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#powered: true,
                r#note: 10,
            });
        }
        if state_id == 1013 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Chime,
                r#note: 16,
            });
        }
        if state_id == 866 {
            return Some(NoteBlock {
                r#note: 17,
                r#instrument: Instrument::Flute,
                r#powered: false,
            });
        }
        if state_id == 1593 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#powered: true,
                r#note: 6,
            });
        }
        if state_id == 701 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#powered: true,
                r#note: 10,
            });
        }
        if state_id == 1512 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 15,
                r#powered: false,
            });
        }
        if state_id == 1666 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 17,
                r#powered: false,
            });
        }
        if state_id == 1612 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::WitherSkeleton,
                r#note: 15,
            });
        }
        if state_id == 1721 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 20,
                r#powered: true,
            });
        }
        if state_id == 758 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 13,
                r#powered: false,
            });
        }
        if state_id == 1550 {
            return Some(NoteBlock {
                r#note: 9,
                r#instrument: Instrument::Dragon,
                r#powered: false,
            });
        }
        if state_id == 880 {
            return Some(NoteBlock {
                r#note: 24,
                r#instrument: Instrument::Flute,
                r#powered: false,
            });
        }
        if state_id == 629 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 24,
                r#powered: true,
            });
        }
        if state_id == 714 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Snare,
                r#note: 16,
            });
        }
        if state_id == 639 {
            return Some(NoteBlock {
                r#note: 4,
                r#instrument: Instrument::Basedrum,
                r#powered: true,
            });
        }
        if state_id == 1012 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#note: 15,
                r#powered: false,
            });
        }
        if state_id == 1633 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 1,
                r#powered: true,
            });
        }
        if state_id == 1695 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 7,
                r#instrument: Instrument::CustomHead,
            });
        }
        if state_id == 746 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Hat,
                r#note: 7,
            });
        }
        if state_id == 730 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 24,
                r#powered: false,
            });
        }
        if state_id == 1654 {
            return Some(NoteBlock {
                r#note: 11,
                r#powered: false,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 1660 {
            return Some(NoteBlock {
                r#note: 14,
                r#powered: false,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 1392 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#note: 5,
                r#powered: false,
            });
        }
        if state_id == 1020 {
            return Some(NoteBlock {
                r#note: 19,
                r#powered: false,
                r#instrument: Instrument::Chime,
            });
        }
        if state_id == 819 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 19,
                r#powered: true,
            });
        }
        if state_id == 1387 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Zombie,
                r#note: 3,
            });
        }
        if state_id == 1203 {
            return Some(NoteBlock {
                r#note: 11,
                r#powered: true,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 837 {
            return Some(NoteBlock {
                r#note: 3,
                r#instrument: Instrument::Flute,
                r#powered: true,
            });
        }
        if state_id == 1527 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 23,
                r#powered: true,
            });
        }
        if state_id == 706 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 12,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 617 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 18,
                r#powered: true,
            });
        }
        if state_id == 1216 {
            return Some(NoteBlock {
                r#note: 17,
                r#powered: false,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 1223 {
            return Some(NoteBlock {
                r#note: 21,
                r#powered: true,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 1291 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 5,
                r#powered: true,
            });
        }
        if state_id == 770 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 19,
                r#powered: false,
            });
        }
        if state_id == 1103 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 11,
                r#powered: true,
            });
        }
        if state_id == 1195 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Didgeridoo,
                r#note: 7,
            });
        }
        if state_id == 1307 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Banjo,
                r#note: 13,
            });
        }
        if state_id == 1517 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#powered: true,
                r#note: 18,
            });
        }
        if state_id == 1549 {
            return Some(NoteBlock {
                r#note: 9,
                r#instrument: Instrument::Dragon,
                r#powered: true,
            });
        }
        if state_id == 973 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 21,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 1627 {
            return Some(NoteBlock {
                r#note: 23,
                r#instrument: Instrument::WitherSkeleton,
                r#powered: true,
            });
        }
        if state_id == 1135 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::CowBell,
                r#note: 2,
            });
        }
        if state_id == 1665 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#powered: true,
                r#note: 17,
            });
        }
        if state_id == 1048 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#note: 8,
                r#powered: false,
            });
        }
        if state_id == 1096 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#powered: false,
                r#note: 7,
            });
        }
        if state_id == 1127 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#powered: true,
                r#note: 23,
            });
        }
        if state_id == 1033 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Xylophone,
                r#note: 1,
            });
        }
        if state_id == 1409 {
            return Some(NoteBlock {
                r#note: 14,
                r#powered: true,
                r#instrument: Instrument::Zombie,
            });
        }
        if state_id == 641 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Basedrum,
                r#note: 5,
            });
        }
        if state_id == 1196 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Didgeridoo,
                r#note: 7,
            });
        }
        if state_id == 1278 {
            return Some(NoteBlock {
                r#note: 23,
                r#instrument: Instrument::Bit,
                r#powered: false,
            });
        }
        if state_id == 1484 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 1,
                r#powered: false,
            });
        }
        if state_id == 1597 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::WitherSkeleton,
                r#note: 8,
            });
        }
        if state_id == 1306 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 12,
                r#powered: false,
            });
        }
        if state_id == 1539 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#note: 4,
                r#powered: true,
            });
        }
        if state_id == 971 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Guitar,
                r#note: 20,
            });
        }
        if state_id == 1280 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 24,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 1553 {
            return Some(NoteBlock {
                r#note: 11,
                r#instrument: Instrument::Dragon,
                r#powered: true,
            });
        }
        if state_id == 1562 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 15,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 908 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bell,
                r#note: 13,
            });
        }
        if state_id == 793 {
            return Some(NoteBlock {
                r#note: 6,
                r#powered: true,
                r#instrument: Instrument::Bass,
            });
        }
        if state_id == 1145 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#note: 7,
                r#powered: true,
            });
        }
        if state_id == 619 {
            return Some(NoteBlock {
                r#note: 19,
                r#instrument: Instrument::Harp,
                r#powered: true,
            });
        }
        if state_id == 1173 {
            return Some(NoteBlock {
                r#note: 21,
                r#instrument: Instrument::CowBell,
                r#powered: true,
            });
        }
        if state_id == 1202 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Didgeridoo,
                r#note: 10,
            });
        }
        if state_id == 1305 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 12,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 729 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 24,
                r#powered: true,
            });
        }
        if state_id == 856 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Flute,
                r#note: 12,
            });
        }
        if state_id == 863 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 16,
                r#powered: true,
            });
        }
        if state_id == 1112 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::IronXylophone,
                r#note: 15,
            });
        }
        if state_id == 840 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Flute,
                r#note: 4,
            });
        }
        if state_id == 1497 {
            return Some(NoteBlock {
                r#note: 8,
                r#powered: true,
                r#instrument: Instrument::Creeper,
            });
        }
        if state_id == 1157 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#powered: true,
                r#note: 13,
            });
        }
        if state_id == 1285 {
            return Some(NoteBlock {
                r#note: 2,
                r#powered: true,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 1443 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#powered: true,
                r#note: 6,
            });
        }
        if state_id == 642 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 5,
                r#powered: false,
            });
        }
        if state_id == 1371 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#powered: true,
                r#note: 20,
            });
        }
        if state_id == 1438 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 3,
                r#instrument: Instrument::Skeleton,
            });
        }
        if state_id == 1461 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Skeleton,
                r#note: 15,
            });
        }
        if state_id == 1198 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#note: 8,
                r#powered: false,
            });
        }
        if state_id == 1004 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#note: 11,
                r#powered: false,
            });
        }
        if state_id == 990 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#powered: false,
                r#note: 4,
            });
        }
        if state_id == 828 {
            return Some(NoteBlock {
                r#note: 23,
                r#instrument: Instrument::Bass,
                r#powered: false,
            });
        }
        if state_id == 1244 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#powered: false,
                r#note: 6,
            });
        }
        if state_id == 594 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 6,
                r#powered: false,
            });
        }
        if state_id == 665 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 17,
                r#powered: true,
            });
        }
        if state_id == 1093 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 6,
                r#powered: true,
            });
        }
        if state_id == 1594 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 6,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 1727 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 23,
                r#powered: true,
            });
        }
        if state_id == 1412 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#note: 15,
                r#powered: false,
            });
        }
        if state_id == 839 {
            return Some(NoteBlock {
                r#note: 4,
                r#powered: true,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 791 {
            return Some(NoteBlock {
                r#note: 5,
                r#powered: true,
                r#instrument: Instrument::Bass,
            });
        }
        if state_id == 1018 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#powered: false,
                r#note: 18,
            });
        }
        if state_id == 753 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 11,
                r#powered: true,
            });
        }
        if state_id == 877 {
            return Some(NoteBlock {
                r#note: 23,
                r#instrument: Instrument::Flute,
                r#powered: true,
            });
        }
        if state_id == 1469 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#powered: true,
                r#note: 19,
            });
        }
        if state_id == 1523 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 21,
                r#powered: true,
            });
        }
        if state_id == 1639 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 4,
                r#powered: true,
            });
        }
        if state_id == 794 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bass,
                r#note: 6,
            });
        }
        if state_id == 1215 {
            return Some(NoteBlock {
                r#note: 17,
                r#powered: true,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 1551 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Dragon,
                r#note: 10,
            });
        }
        if state_id == 1701 {
            return Some(NoteBlock {
                r#note: 10,
                r#powered: true,
                r#instrument: Instrument::CustomHead,
            });
        }
        if state_id == 1095 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 7,
                r#powered: true,
            });
        }
        if state_id == 1232 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#note: 0,
                r#powered: false,
            });
        }
        if state_id == 1254 {
            return Some(NoteBlock {
                r#note: 11,
                r#powered: false,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 1625 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::WitherSkeleton,
                r#note: 22,
            });
        }
        if state_id == 1234 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 1,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 1626 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::WitherSkeleton,
                r#note: 22,
            });
        }
        if state_id == 1241 {
            return Some(NoteBlock {
                r#note: 5,
                r#powered: true,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 847 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 8,
                r#powered: true,
            });
        }
        if state_id == 1174 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 21,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 1039 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Xylophone,
                r#note: 4,
            });
        }
        if state_id == 1250 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bit,
                r#note: 9,
            });
        }
        if state_id == 1540 {
            return Some(NoteBlock {
                r#note: 4,
                r#instrument: Instrument::Dragon,
                r#powered: false,
            });
        }
        if state_id == 870 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 19,
                r#powered: false,
            });
        }
        if state_id == 1042 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#note: 5,
                r#powered: false,
            });
        }
        if state_id == 903 {
            return Some(NoteBlock {
                r#note: 11,
                r#powered: true,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 1211 {
            return Some(NoteBlock {
                r#note: 15,
                r#powered: true,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 926 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#powered: false,
                r#note: 22,
            });
        }
        if state_id == 911 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#note: 15,
                r#powered: true,
            });
        }
        if state_id == 1342 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Pling,
                r#note: 5,
            });
        }
        if state_id == 1726 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 22,
                r#powered: false,
            });
        }
        if state_id == 694 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 6,
                r#powered: false,
            });
        }
        if state_id == 1586 {
            return Some(NoteBlock {
                r#note: 2,
                r#instrument: Instrument::WitherSkeleton,
                r#powered: false,
            });
        }
        if state_id == 1600 {
            return Some(NoteBlock {
                r#note: 9,
                r#powered: false,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 1578 {
            return Some(NoteBlock {
                r#note: 23,
                r#instrument: Instrument::Dragon,
                r#powered: false,
            });
        }
        if state_id == 676 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 22,
                r#powered: false,
            });
        }
        if state_id == 1151 {
            return Some(NoteBlock {
                r#note: 10,
                r#instrument: Instrument::CowBell,
                r#powered: true,
            });
        }
        if state_id == 609 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#powered: true,
                r#note: 14,
            });
        }
        if state_id == 996 {
            return Some(NoteBlock {
                r#note: 7,
                r#instrument: Instrument::Chime,
                r#powered: false,
            });
        }
        if state_id == 860 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 14,
                r#powered: false,
            });
        }
        if state_id == 914 {
            return Some(NoteBlock {
                r#note: 16,
                r#powered: false,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 618 {
            return Some(NoteBlock {
                r#note: 18,
                r#powered: false,
                r#instrument: Instrument::Harp,
            });
        }
        if state_id == 1323 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 21,
                r#powered: true,
            });
        }
        if state_id == 1340 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#powered: false,
                r#note: 4,
            });
        }
        if state_id == 846 {
            return Some(NoteBlock {
                r#note: 7,
                r#powered: false,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 1376 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Pling,
                r#note: 22,
            });
        }
        if state_id == 1534 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Dragon,
                r#note: 1,
            });
        }
        if state_id == 1372 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Pling,
                r#note: 20,
            });
        }
        if state_id == 622 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 20,
                r#powered: false,
            });
        }
        if state_id == 1063 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#note: 16,
                r#powered: true,
            });
        }
        if state_id == 1359 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#powered: true,
                r#note: 14,
            });
        }
        if state_id == 983 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 1,
                r#instrument: Instrument::Chime,
            });
        }
        if state_id == 702 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 10,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 1542 {
            return Some(NoteBlock {
                r#note: 5,
                r#powered: false,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 811 {
            return Some(NoteBlock {
                r#note: 15,
                r#powered: true,
                r#instrument: Instrument::Bass,
            });
        }
        if state_id == 916 {
            return Some(NoteBlock {
                r#note: 17,
                r#powered: false,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 1164 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#powered: false,
                r#note: 16,
            });
        }
        if state_id == 1081 {
            return Some(NoteBlock {
                r#note: 0,
                r#powered: true,
                r#instrument: Instrument::IronXylophone,
            });
        }
        if state_id == 1114 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 16,
                r#powered: false,
            });
        }
        if state_id == 1329 {
            return Some(NoteBlock {
                r#note: 24,
                r#powered: true,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 988 {
            return Some(NoteBlock {
                r#note: 3,
                r#instrument: Instrument::Chime,
                r#powered: false,
            });
        }
        if state_id == 1415 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#note: 17,
                r#powered: true,
            });
        }
        if state_id == 1083 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::IronXylophone,
                r#note: 1,
            });
        }
        if state_id == 786 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bass,
                r#note: 2,
            });
        }
        if state_id == 1339 {
            return Some(NoteBlock {
                r#note: 4,
                r#powered: true,
                r#instrument: Instrument::Pling,
            });
        }
        if state_id == 1070 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#note: 19,
                r#powered: false,
            });
        }
        if state_id == 722 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Snare,
                r#note: 20,
            });
        }
        if state_id == 1441 {
            return Some(NoteBlock {
                r#note: 5,
                r#instrument: Instrument::Skeleton,
                r#powered: true,
            });
        }
        if state_id == 1529 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Creeper,
                r#note: 24,
            });
        }
        if state_id == 1650 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Piglin,
                r#note: 9,
            });
        }
        if state_id == 719 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Snare,
                r#note: 19,
            });
        }
        if state_id == 1334 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 1,
                r#instrument: Instrument::Pling,
            });
        }
        if state_id == 1385 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 2,
                r#instrument: Instrument::Zombie,
            });
        }
        if state_id == 1456 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 12,
                r#instrument: Instrument::Skeleton,
            });
        }
        if state_id == 1485 {
            return Some(NoteBlock {
                r#note: 2,
                r#powered: true,
                r#instrument: Instrument::Creeper,
            });
        }
        if state_id == 1667 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 18,
                r#powered: true,
            });
        }
        if state_id == 1247 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#powered: true,
                r#note: 8,
            });
        }
        if state_id == 891 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#note: 5,
                r#powered: true,
            });
        }
        if state_id == 1470 {
            return Some(NoteBlock {
                r#note: 19,
                r#powered: false,
                r#instrument: Instrument::Skeleton,
            });
        }
        if state_id == 893 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bell,
                r#note: 6,
            });
        }
        if state_id == 905 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bell,
                r#note: 12,
            });
        }
        if state_id == 1475 {
            return Some(NoteBlock {
                r#note: 22,
                r#powered: true,
                r#instrument: Instrument::Skeleton,
            });
        }
        if state_id == 636 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 2,
                r#instrument: Instrument::Basedrum,
            });
        }
        if state_id == 693 {
            return Some(NoteBlock {
                r#note: 6,
                r#instrument: Instrument::Snare,
                r#powered: true,
            });
        }
        if state_id == 1428 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Zombie,
                r#note: 23,
            });
        }
        if state_id == 1513 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 16,
                r#powered: true,
            });
        }
        if state_id == 985 {
            return Some(NoteBlock {
                r#note: 2,
                r#powered: true,
                r#instrument: Instrument::Chime,
            });
        }
        if state_id == 808 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#powered: false,
                r#note: 13,
            });
        }
        if state_id == 1698 {
            return Some(NoteBlock {
                r#note: 8,
                r#instrument: Instrument::CustomHead,
                r#powered: false,
            });
        }
        if state_id == 780 {
            return Some(NoteBlock {
                r#note: 24,
                r#instrument: Instrument::Hat,
                r#powered: false,
            });
        }
        if state_id == 1162 {
            return Some(NoteBlock {
                r#note: 15,
                r#powered: false,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 759 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 14,
                r#powered: true,
            });
        }
        if state_id == 789 {
            return Some(NoteBlock {
                r#note: 4,
                r#instrument: Instrument::Bass,
                r#powered: true,
            });
        }
        if state_id == 826 {
            return Some(NoteBlock {
                r#note: 22,
                r#instrument: Instrument::Bass,
                r#powered: false,
            });
        }
        if state_id == 1218 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 18,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 1632 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 0,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 946 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Guitar,
                r#note: 7,
            });
        }
        if state_id == 1348 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 8,
                r#instrument: Instrument::Pling,
            });
        }
        if state_id == 728 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Snare,
                r#note: 23,
            });
        }
        if state_id == 680 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#powered: false,
                r#note: 24,
            });
        }
        if state_id == 590 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Harp,
                r#note: 4,
            });
        }
        if state_id == 1476 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#note: 22,
                r#powered: false,
            });
        }
        if state_id == 944 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 6,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 744 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 6,
                r#instrument: Instrument::Hat,
            });
        }
        if state_id == 902 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#powered: false,
                r#note: 10,
            });
        }
        if state_id == 1170 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#note: 19,
                r#powered: false,
            });
        }
        if state_id == 1655 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 12,
                r#powered: true,
            });
        }
        if state_id == 703 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Snare,
                r#note: 11,
            });
        }
        if state_id == 940 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Guitar,
                r#note: 4,
            });
        }
        if state_id == 1569 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 19,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 607 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 13,
                r#instrument: Instrument::Harp,
            });
        }
        if state_id == 1680 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Piglin,
                r#note: 24,
            });
        }
        if state_id == 1684 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 1,
                r#powered: false,
            });
        }
        if state_id == 1360 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#note: 14,
                r#powered: false,
            });
        }
        if state_id == 1413 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#note: 16,
                r#powered: true,
            });
        }
        if state_id == 670 {
            return Some(NoteBlock {
                r#note: 19,
                r#instrument: Instrument::Basedrum,
                r#powered: false,
            });
        }
        if state_id == 841 {
            return Some(NoteBlock {
                r#note: 5,
                r#powered: true,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 1003 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Chime,
                r#note: 11,
            });
        }
        if state_id == 1703 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::CustomHead,
                r#note: 11,
            });
        }
        if state_id == 1519 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 19,
                r#powered: true,
            });
        }
        if state_id == 1022 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#note: 20,
                r#powered: false,
            });
        }
        if state_id == 742 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#powered: false,
                r#note: 5,
            });
        }
        if state_id == 1304 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 11,
                r#powered: false,
            });
        }
        if state_id == 713 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Snare,
                r#note: 16,
            });
        }
        if state_id == 700 {
            return Some(NoteBlock {
                r#note: 9,
                r#powered: false,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 581 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#powered: true,
                r#note: 0,
            });
        }
        if state_id == 1588 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#powered: false,
                r#note: 3,
            });
        }
        if state_id == 1059 {
            return Some(NoteBlock {
                r#note: 14,
                r#powered: true,
                r#instrument: Instrument::Xylophone,
            });
        }
        if state_id == 1117 {
            return Some(NoteBlock {
                r#note: 18,
                r#instrument: Instrument::IronXylophone,
                r#powered: true,
            });
        }
        if state_id == 1287 {
            return Some(NoteBlock {
                r#note: 3,
                r#instrument: Instrument::Banjo,
                r#powered: true,
            });
        }
        if state_id == 1294 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 6,
                r#powered: false,
            });
        }
        if state_id == 1439 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Skeleton,
                r#note: 4,
            });
        }
        if state_id == 778 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 23,
                r#powered: false,
            });
        }
        if state_id == 1209 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 14,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 691 {
            return Some(NoteBlock {
                r#note: 5,
                r#powered: true,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 1275 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#powered: true,
                r#note: 22,
            });
        }
        if state_id == 1610 {
            return Some(NoteBlock {
                r#note: 14,
                r#instrument: Instrument::WitherSkeleton,
                r#powered: false,
            });
        }
        if state_id == 1690 {
            return Some(NoteBlock {
                r#note: 4,
                r#powered: false,
                r#instrument: Instrument::CustomHead,
            });
        }
        if state_id == 1431 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#note: 0,
                r#powered: true,
            });
        }
        if state_id == 1580 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 24,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 1076 {
            return Some(NoteBlock {
                r#note: 22,
                r#instrument: Instrument::Xylophone,
                r#powered: false,
            });
        }
        if state_id == 644 {
            return Some(NoteBlock {
                r#note: 6,
                r#powered: false,
                r#instrument: Instrument::Basedrum,
            });
        }
        if state_id == 767 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 18,
                r#powered: true,
            });
        }
        if state_id == 627 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#powered: true,
                r#note: 23,
            });
        }
        if state_id == 783 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bass,
                r#note: 1,
            });
        }
        if state_id == 1525 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 22,
                r#powered: true,
            });
        }
        if state_id == 748 {
            return Some(NoteBlock {
                r#note: 8,
                r#powered: false,
                r#instrument: Instrument::Hat,
            });
        }
        if state_id == 782 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#powered: false,
                r#note: 0,
            });
        }
        if state_id == 1444 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Skeleton,
                r#note: 6,
            });
        }
        if state_id == 1704 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::CustomHead,
                r#note: 11,
            });
        }
        if state_id == 591 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 5,
                r#powered: true,
            });
        }
        if state_id == 1712 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#powered: false,
                r#note: 15,
            });
        }
        if state_id == 1536 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Dragon,
                r#note: 2,
            });
        }
        if state_id == 1182 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Didgeridoo,
                r#note: 0,
            });
        }
        if state_id == 1468 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 18,
                r#instrument: Instrument::Skeleton,
            });
        }
        if state_id == 795 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bass,
                r#note: 7,
            });
        }
        if state_id == 597 {
            return Some(NoteBlock {
                r#note: 8,
                r#instrument: Instrument::Harp,
                r#powered: true,
            });
        }
        if state_id == 690 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 4,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 1630 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#powered: false,
                r#note: 24,
            });
        }
        if state_id == 1486 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 2,
                r#instrument: Instrument::Creeper,
            });
        }
        if state_id == 1156 {
            return Some(NoteBlock {
                r#note: 12,
                r#instrument: Instrument::CowBell,
                r#powered: false,
            });
        }
        if state_id == 699 {
            return Some(NoteBlock {
                r#note: 9,
                r#powered: true,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 781 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bass,
                r#note: 0,
            });
        }
        if state_id == 1255 {
            return Some(NoteBlock {
                r#note: 12,
                r#powered: true,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 972 {
            return Some(NoteBlock {
                r#note: 20,
                r#powered: false,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 659 {
            return Some(NoteBlock {
                r#note: 14,
                r#instrument: Instrument::Basedrum,
                r#powered: true,
            });
        }
        if state_id == 763 {
            return Some(NoteBlock {
                r#note: 16,
                r#instrument: Instrument::Hat,
                r#powered: true,
            });
        }
        if state_id == 1543 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#note: 6,
                r#powered: true,
            });
        }
        if state_id == 621 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Harp,
                r#note: 20,
            });
        }
        if state_id == 1730 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 24,
                r#powered: false,
            });
        }
        if state_id == 1000 {
            return Some(NoteBlock {
                r#note: 9,
                r#instrument: Instrument::Chime,
                r#powered: false,
            });
        }
        if state_id == 1488 {
            return Some(NoteBlock {
                r#note: 3,
                r#powered: false,
                r#instrument: Instrument::Creeper,
            });
        }
        if state_id == 673 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 21,
                r#instrument: Instrument::Basedrum,
            });
        }
        if state_id == 1057 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#note: 13,
                r#powered: true,
            });
        }
        if state_id == 1368 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#note: 18,
                r#powered: false,
            });
        }
        if state_id == 825 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bass,
                r#note: 22,
            });
        }
        if state_id == 1407 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Zombie,
                r#note: 13,
            });
        }
        if state_id == 1034 {
            return Some(NoteBlock {
                r#note: 1,
                r#instrument: Instrument::Xylophone,
                r#powered: false,
            });
        }
        if state_id == 1631 {
            return Some(NoteBlock {
                r#note: 0,
                r#powered: true,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 1224 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 21,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 1656 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Piglin,
                r#note: 12,
            });
        }
        if state_id == 685 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 2,
                r#powered: true,
            });
        }
        if state_id == 965 {
            return Some(NoteBlock {
                r#note: 17,
                r#instrument: Instrument::Guitar,
                r#powered: true,
            });
        }
        if state_id == 1222 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Didgeridoo,
                r#note: 20,
            });
        }
        if state_id == 1560 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#powered: false,
                r#note: 14,
            });
        }
        if state_id == 1602 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#note: 10,
                r#powered: false,
            });
        }
        if state_id == 1573 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Dragon,
                r#note: 21,
            });
        }
        if state_id == 855 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Flute,
                r#note: 12,
            });
        }
        if state_id == 1179 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#note: 24,
                r#powered: true,
            });
        }
        if state_id == 1016 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Chime,
                r#note: 17,
            });
        }
        if state_id == 1361 {
            return Some(NoteBlock {
                r#note: 15,
                r#powered: true,
                r#instrument: Instrument::Pling,
            });
        }
        if state_id == 1545 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#note: 7,
                r#powered: true,
            });
        }
        if state_id == 740 {
            return Some(NoteBlock {
                r#note: 4,
                r#instrument: Instrument::Hat,
                r#powered: false,
            });
        }
        if state_id == 645 {
            return Some(NoteBlock {
                r#note: 7,
                r#instrument: Instrument::Basedrum,
                r#powered: true,
            });
        }
        if state_id == 917 {
            return Some(NoteBlock {
                r#note: 18,
                r#powered: true,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 1243 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#powered: true,
                r#note: 6,
            });
        }
        if state_id == 1051 {
            return Some(NoteBlock {
                r#note: 10,
                r#instrument: Instrument::Xylophone,
                r#powered: true,
            });
        }
        if state_id == 585 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Harp,
                r#note: 2,
            });
        }
        if state_id == 1248 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bit,
                r#note: 8,
            });
        }
        if state_id == 1449 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#note: 9,
                r#powered: true,
            });
        }
        if state_id == 1425 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#note: 22,
                r#powered: true,
            });
        }
        if state_id == 1478 {
            return Some(NoteBlock {
                r#note: 23,
                r#instrument: Instrument::Skeleton,
                r#powered: false,
            });
        }
        if state_id == 1556 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#note: 12,
                r#powered: false,
            });
        }
        if state_id == 1564 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Dragon,
                r#note: 16,
            });
        }
        if state_id == 745 {
            return Some(NoteBlock {
                r#note: 7,
                r#powered: true,
                r#instrument: Instrument::Hat,
            });
        }
        if state_id == 922 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bell,
                r#note: 20,
            });
        }
        if state_id == 1405 {
            return Some(NoteBlock {
                r#instrument: Instrument::Zombie,
                r#powered: true,
                r#note: 12,
            });
        }
        if state_id == 1446 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Skeleton,
                r#note: 7,
            });
        }
        if state_id == 1609 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#note: 14,
                r#powered: true,
            });
        }
        if state_id == 1166 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#note: 17,
                r#powered: false,
            });
        }
        if state_id == 1272 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bit,
                r#note: 20,
            });
        }
        if state_id == 595 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 7,
                r#powered: true,
            });
        }
        if state_id == 625 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 22,
                r#powered: true,
            });
        }
        if state_id == 982 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Chime,
                r#note: 0,
            });
        }
        if state_id == 687 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 3,
                r#powered: true,
            });
        }
        if state_id == 1043 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Xylophone,
                r#note: 6,
            });
        }
        if state_id == 1251 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 10,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 637 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 3,
                r#powered: true,
            });
        }
        if state_id == 1605 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#note: 12,
                r#powered: true,
            });
        }
        if state_id == 1011 {
            return Some(NoteBlock {
                r#note: 15,
                r#powered: true,
                r#instrument: Instrument::Chime,
            });
        }
        if state_id == 1584 {
            return Some(NoteBlock {
                r#note: 1,
                r#instrument: Instrument::WitherSkeleton,
                r#powered: false,
            });
        }
        if state_id == 658 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Basedrum,
                r#note: 13,
            });
        }
        if state_id == 879 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 24,
                r#powered: true,
            });
        }
        if state_id == 1121 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::IronXylophone,
                r#note: 20,
            });
        }
        if state_id == 1242 {
            return Some(NoteBlock {
                r#note: 5,
                r#powered: false,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 1516 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 17,
                r#instrument: Instrument::Creeper,
            });
        }
        if state_id == 1663 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 16,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 1301 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Banjo,
                r#note: 10,
            });
        }
        if state_id == 1235 {
            return Some(NoteBlock {
                r#note: 2,
                r#instrument: Instrument::Bit,
                r#powered: true,
            });
        }
        if state_id == 976 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#note: 22,
                r#powered: false,
            });
        }
        if state_id == 1404 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 11,
                r#instrument: Instrument::Zombie,
            });
        }
        if state_id == 878 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 23,
                r#powered: false,
            });
        }
        if state_id == 1501 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 10,
                r#instrument: Instrument::Creeper,
            });
        }
        if state_id == 1575 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#powered: true,
                r#note: 22,
            });
        }
        if state_id == 1638 {
            return Some(NoteBlock {
                r#note: 3,
                r#powered: false,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 1032 {
            return Some(NoteBlock {
                r#note: 0,
                r#instrument: Instrument::Xylophone,
                r#powered: false,
            });
        }
        if state_id == 588 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 3,
                r#instrument: Instrument::Harp,
            });
        }
        if state_id == 1436 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#powered: false,
                r#note: 2,
            });
        }
        if state_id == 716 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 17,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 924 {
            return Some(NoteBlock {
                r#note: 21,
                r#powered: false,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 1106 {
            return Some(NoteBlock {
                r#note: 12,
                r#instrument: Instrument::IronXylophone,
                r#powered: false,
            });
        }
        if state_id == 1398 {
            return Some(NoteBlock {
                r#note: 8,
                r#powered: false,
                r#instrument: Instrument::Zombie,
            });
        }
        if state_id == 1688 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#powered: false,
                r#note: 3,
            });
        }
        if state_id == 610 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 14,
                r#instrument: Instrument::Harp,
            });
        }
        if state_id == 1642 {
            return Some(NoteBlock {
                r#note: 5,
                r#instrument: Instrument::Piglin,
                r#powered: false,
            });
        }
        if state_id == 1614 {
            return Some(NoteBlock {
                r#note: 16,
                r#instrument: Instrument::WitherSkeleton,
                r#powered: false,
            });
        }
        if state_id == 947 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 8,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 1473 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Skeleton,
                r#note: 21,
            });
        }
        if state_id == 1284 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 1,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 1440 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Skeleton,
                r#note: 4,
            });
        }
        if state_id == 660 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 14,
                r#powered: false,
            });
        }
        if state_id == 1199 {
            return Some(NoteBlock {
                r#note: 9,
                r#powered: true,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 835 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 2,
                r#powered: true,
            });
        }
        if state_id == 640 {
            return Some(NoteBlock {
                r#note: 4,
                r#powered: false,
                r#instrument: Instrument::Basedrum,
            });
        }
        if state_id == 592 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Harp,
                r#note: 5,
            });
        }
        if state_id == 1532 {
            return Some(NoteBlock {
                r#note: 0,
                r#powered: false,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 1087 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::IronXylophone,
                r#note: 3,
            });
        }
        if state_id == 1622 {
            return Some(NoteBlock {
                r#note: 20,
                r#powered: false,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 1641 {
            return Some(NoteBlock {
                r#note: 5,
                r#powered: true,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 1077 {
            return Some(NoteBlock {
                r#note: 23,
                r#powered: true,
                r#instrument: Instrument::Xylophone,
            });
        }
        if state_id == 589 {
            return Some(NoteBlock {
                r#note: 4,
                r#powered: true,
                r#instrument: Instrument::Harp,
            });
        }
        if state_id == 1598 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::WitherSkeleton,
                r#note: 8,
            });
        }
        if state_id == 1125 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 22,
                r#powered: true,
            });
        }
        if state_id == 894 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#note: 6,
                r#powered: false,
            });
        }
        if state_id == 881 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bell,
                r#note: 0,
            });
        }
        if state_id == 717 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Snare,
                r#note: 18,
            });
        }
        if state_id == 1353 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#powered: true,
                r#note: 11,
            });
        }
        if state_id == 1725 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#powered: true,
                r#note: 22,
            });
        }
        if state_id == 1082 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::IronXylophone,
                r#note: 0,
            });
        }
        if state_id == 604 {
            return Some(NoteBlock {
                r#note: 11,
                r#powered: false,
                r#instrument: Instrument::Harp,
            });
        }
        if state_id == 1030 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 24,
                r#instrument: Instrument::Chime,
            });
        }
        if state_id == 1434 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#note: 1,
                r#powered: false,
            });
        }
        if state_id == 777 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 23,
                r#powered: true,
            });
        }
        if state_id == 929 {
            return Some(NoteBlock {
                r#note: 24,
                r#instrument: Instrument::Bell,
                r#powered: true,
            });
        }
        if state_id == 1320 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 19,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 731 {
            return Some(NoteBlock {
                r#note: 0,
                r#instrument: Instrument::Hat,
                r#powered: true,
            });
        }
        if state_id == 1397 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Zombie,
                r#note: 8,
            });
        }
        if state_id == 1528 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 23,
                r#powered: false,
            });
        }
        if state_id == 1716 {
            return Some(NoteBlock {
                r#note: 17,
                r#instrument: Instrument::CustomHead,
                r#powered: false,
            });
        }
        if state_id == 932 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 0,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 907 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#powered: true,
                r#note: 13,
            });
        }
        if state_id == 964 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Guitar,
                r#note: 16,
            });
        }
        if state_id == 1708 {
            return Some(NoteBlock {
                r#note: 13,
                r#powered: false,
                r#instrument: Instrument::CustomHead,
            });
        }
        if state_id == 850 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 9,
                r#powered: false,
            });
        }
        if state_id == 1253 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 11,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 766 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#powered: false,
                r#note: 17,
            });
        }
        if state_id == 1185 {
            return Some(NoteBlock {
                r#note: 2,
                r#instrument: Instrument::Didgeridoo,
                r#powered: true,
            });
        }
        if state_id == 1298 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#powered: false,
                r#note: 8,
            });
        }
        if state_id == 1105 {
            return Some(NoteBlock {
                r#note: 12,
                r#powered: true,
                r#instrument: Instrument::IronXylophone,
            });
        }
        if state_id == 738 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 3,
                r#instrument: Instrument::Hat,
            });
        }
        if state_id == 1295 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Banjo,
                r#note: 7,
            });
        }
        if state_id == 1321 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 20,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 615 {
            return Some(NoteBlock {
                r#note: 17,
                r#instrument: Instrument::Harp,
                r#powered: true,
            });
        }
        if state_id == 1601 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#note: 10,
                r#powered: true,
            });
        }
        if state_id == 1335 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#note: 2,
                r#powered: true,
            });
        }
        if state_id == 1435 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#note: 2,
                r#powered: true,
            });
        }
        if state_id == 1333 {
            return Some(NoteBlock {
                r#note: 1,
                r#instrument: Instrument::Pling,
                r#powered: true,
            });
        }
        if state_id == 830 {
            return Some(NoteBlock {
                r#note: 24,
                r#powered: false,
                r#instrument: Instrument::Bass,
            });
        }
        if state_id == 1002 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Chime,
                r#note: 10,
            });
        }
        if state_id == 1147 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#powered: true,
                r#note: 8,
            });
        }
        if state_id == 975 {
            return Some(NoteBlock {
                r#note: 22,
                r#instrument: Instrument::Guitar,
                r#powered: true,
            });
        }
        if state_id == 734 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 1,
                r#instrument: Instrument::Hat,
            });
        }
        if state_id == 812 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bass,
                r#note: 15,
            });
        }
        if state_id == 889 {
            return Some(NoteBlock {
                r#note: 4,
                r#instrument: Instrument::Bell,
                r#powered: true,
            });
        }
        if state_id == 755 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 12,
                r#powered: true,
            });
        }
        if state_id == 899 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bell,
                r#note: 9,
            });
        }
        if state_id == 683 {
            return Some(NoteBlock {
                r#note: 1,
                r#powered: true,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 1318 {
            return Some(NoteBlock {
                r#note: 18,
                r#powered: false,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 1459 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Skeleton,
                r#note: 14,
            });
        }
        if state_id == 1210 {
            return Some(NoteBlock {
                r#note: 14,
                r#instrument: Instrument::Didgeridoo,
                r#powered: false,
            });
        }
        if state_id == 1369 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Pling,
                r#note: 19,
            });
        }
        if state_id == 623 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Harp,
                r#note: 21,
            });
        }
        if state_id == 824 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#powered: false,
                r#note: 21,
            });
        }
        if state_id == 920 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#note: 19,
                r#powered: false,
            });
        }
        if state_id == 1245 {
            return Some(NoteBlock {
                r#note: 7,
                r#instrument: Instrument::Bit,
                r#powered: true,
            });
        }
        if state_id == 612 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 15,
                r#instrument: Instrument::Harp,
            });
        }
        if state_id == 715 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#powered: true,
                r#note: 17,
            });
        }
        if state_id == 853 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 11,
                r#powered: true,
            });
        }
        if state_id == 938 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Guitar,
                r#note: 3,
            });
        }
        if state_id == 667 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 18,
                r#powered: true,
            });
        }
        if state_id == 1537 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 3,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 723 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 21,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 898 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#powered: false,
                r#note: 8,
            });
        }
        if state_id == 1069 {
            return Some(NoteBlock {
                r#note: 19,
                r#instrument: Instrument::Xylophone,
                r#powered: true,
            });
        }
        if state_id == 1153 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::CowBell,
                r#note: 11,
            });
        }
        if state_id == 1037 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 3,
                r#instrument: Instrument::Xylophone,
            });
        }
        if state_id == 1167 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#powered: true,
                r#note: 18,
            });
        }
        if state_id == 790 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#powered: false,
                r#note: 4,
            });
        }
        if state_id == 1194 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#note: 6,
                r#powered: false,
            });
        }
        if state_id == 1672 {
            return Some(NoteBlock {
                r#note: 20,
                r#instrument: Instrument::Piglin,
                r#powered: false,
            });
        }
        if state_id == 842 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 5,
                r#powered: false,
            });
        }
        if state_id == 1137 {
            return Some(NoteBlock {
                r#note: 3,
                r#instrument: Instrument::CowBell,
                r#powered: true,
            });
        }
        if state_id == 1685 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 2,
                r#instrument: Instrument::CustomHead,
            });
        }
        if state_id == 1296 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#powered: false,
                r#note: 7,
            });
        }
        if state_id == 1711 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#powered: true,
                r#note: 15,
            });
        }
        if state_id == 1300 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 9,
                r#powered: false,
            });
        }
        if state_id == 1141 {
            return Some(NoteBlock {
                r#note: 5,
                r#instrument: Instrument::CowBell,
                r#powered: true,
            });
        }
        if state_id == 1472 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#powered: false,
                r#note: 20,
            });
        }
        if state_id == 945 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#note: 7,
                r#powered: true,
            });
        }
        if state_id == 1592 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#powered: false,
                r#note: 5,
            });
        }
        if state_id == 1669 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 19,
                r#powered: true,
            });
        }
        if state_id == 935 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 2,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 1236 {
            return Some(NoteBlock {
                r#note: 2,
                r#instrument: Instrument::Bit,
                r#powered: false,
            });
        }
        if state_id == 1453 {
            return Some(NoteBlock {
                r#note: 11,
                r#powered: true,
                r#instrument: Instrument::Skeleton,
            });
        }
        if state_id == 1691 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 5,
                r#powered: true,
            });
        }
        if state_id == 760 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#note: 14,
                r#powered: false,
            });
        }
        if state_id == 1510 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 14,
                r#powered: false,
            });
        }
        if state_id == 1471 {
            return Some(NoteBlock {
                r#instrument: Instrument::Skeleton,
                r#note: 20,
                r#powered: true,
            });
        }
        if state_id == 1495 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#powered: true,
                r#note: 7,
            });
        }
        if state_id == 1634 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Piglin,
                r#note: 1,
            });
        }
        if state_id == 752 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#powered: false,
                r#note: 10,
            });
        }
        if state_id == 1500 {
            return Some(NoteBlock {
                r#note: 9,
                r#instrument: Instrument::Creeper,
                r#powered: false,
            });
        }
        if state_id == 704 {
            return Some(NoteBlock {
                r#note: 11,
                r#instrument: Instrument::Snare,
                r#powered: false,
            });
        }
        if state_id == 1673 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#powered: true,
                r#note: 21,
            });
        }
        if state_id == 820 {
            return Some(NoteBlock {
                r#note: 19,
                r#instrument: Instrument::Bass,
                r#powered: false,
            });
        }
        if state_id == 632 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Basedrum,
                r#note: 0,
            });
        }
        if state_id == 1150 {
            return Some(NoteBlock {
                r#note: 9,
                r#instrument: Instrument::CowBell,
                r#powered: false,
            });
        }
        if state_id == 958 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#note: 13,
                r#powered: false,
            });
        }
        if state_id == 675 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 22,
                r#instrument: Instrument::Basedrum,
            });
        }
        if state_id == 657 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 13,
                r#powered: true,
            });
        }
        if state_id == 848 {
            return Some(NoteBlock {
                r#note: 8,
                r#powered: false,
                r#instrument: Instrument::Flute,
            });
        }
        if state_id == 992 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Chime,
                r#note: 5,
            });
        }
        if state_id == 1603 {
            return Some(NoteBlock {
                r#note: 11,
                r#powered: true,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 1394 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Zombie,
                r#note: 6,
            });
        }
        if state_id == 1694 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 6,
                r#powered: false,
            });
        }
        if state_id == 1266 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#note: 17,
                r#powered: false,
            });
        }
        if state_id == 666 {
            return Some(NoteBlock {
                r#note: 17,
                r#powered: false,
                r#instrument: Instrument::Basedrum,
            });
        }
        if state_id == 664 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#note: 16,
                r#powered: false,
            });
        }
        if state_id == 1263 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#note: 16,
                r#powered: true,
            });
        }
        if state_id == 1640 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Piglin,
                r#note: 4,
            });
        }
        if state_id == 798 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 8,
                r#instrument: Instrument::Bass,
            });
        }
        if state_id == 1709 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::CustomHead,
                r#note: 14,
            });
        }
        if state_id == 1427 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Zombie,
                r#note: 23,
            });
        }
        if state_id == 1180 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#powered: false,
                r#note: 24,
            });
        }
        if state_id == 1576 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 22,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 803 {
            return Some(NoteBlock {
                r#note: 11,
                r#instrument: Instrument::Bass,
                r#powered: true,
            });
        }
        if state_id == 1246 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bit,
                r#note: 7,
            });
        }
        if state_id == 962 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 15,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 1707 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#note: 13,
                r#powered: true,
            });
        }
        if state_id == 897 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#note: 8,
                r#powered: true,
            });
        }
        if state_id == 1050 {
            return Some(NoteBlock {
                r#note: 9,
                r#instrument: Instrument::Xylophone,
                r#powered: false,
            });
        }
        if state_id == 1262 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#note: 15,
                r#powered: false,
            });
        }
        if state_id == 668 {
            return Some(NoteBlock {
                r#note: 18,
                r#powered: false,
                r#instrument: Instrument::Basedrum,
            });
        }
        if state_id == 750 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 9,
                r#instrument: Instrument::Hat,
            });
        }
        if state_id == 646 {
            return Some(NoteBlock {
                r#note: 7,
                r#instrument: Instrument::Basedrum,
                r#powered: false,
            });
        }
        if state_id == 1028 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 23,
                r#instrument: Instrument::Chime,
            });
        }
        if state_id == 1217 {
            return Some(NoteBlock {
                r#note: 18,
                r#powered: true,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 1657 {
            return Some(NoteBlock {
                r#note: 13,
                r#powered: true,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 1720 {
            return Some(NoteBlock {
                r#note: 19,
                r#powered: false,
                r#instrument: Instrument::CustomHead,
            });
        }
        if state_id == 845 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#note: 7,
                r#powered: true,
            });
        }
        if state_id == 695 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 7,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 775 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#powered: true,
                r#note: 22,
            });
        }
        if state_id == 1056 {
            return Some(NoteBlock {
                r#note: 12,
                r#instrument: Instrument::Xylophone,
                r#powered: false,
            });
        }
        if state_id == 1585 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::WitherSkeleton,
                r#note: 2,
            });
        }
        if state_id == 586 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Harp,
                r#note: 2,
            });
        }
        if state_id == 787 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 3,
                r#powered: true,
            });
        }
        if state_id == 788 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 3,
                r#powered: false,
            });
        }
        if state_id == 1005 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Chime,
                r#note: 12,
            });
        }
        if state_id == 1677 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 23,
                r#powered: true,
            });
        }
        if state_id == 1671 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 20,
                r#powered: true,
            });
        }
        if state_id == 979 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Guitar,
                r#note: 24,
            });
        }
        if state_id == 1258 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Bit,
                r#note: 13,
            });
        }
        if state_id == 737 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#powered: true,
                r#note: 3,
            });
        }
        if state_id == 1220 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Didgeridoo,
                r#note: 19,
            });
        }
        if state_id == 1120 {
            return Some(NoteBlock {
                r#note: 19,
                r#powered: false,
                r#instrument: Instrument::IronXylophone,
            });
        }
        if state_id == 1467 {
            return Some(NoteBlock {
                r#note: 18,
                r#instrument: Instrument::Skeleton,
                r#powered: true,
            });
        }
        if state_id == 1572 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Dragon,
                r#note: 20,
            });
        }
        if state_id == 1175 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 22,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 1293 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 6,
                r#powered: true,
            });
        }
        if state_id == 1719 {
            return Some(NoteBlock {
                r#note: 19,
                r#powered: true,
                r#instrument: Instrument::CustomHead,
            });
        }
        if state_id == 952 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#note: 10,
                r#powered: false,
            });
        }
        if state_id == 886 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#note: 2,
                r#powered: false,
            });
        }
        if state_id == 1325 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 22,
                r#powered: true,
            });
        }
        if state_id == 1652 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Piglin,
                r#note: 10,
            });
        }
        if state_id == 948 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#note: 8,
                r#powered: false,
            });
        }
        if state_id == 1155 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::CowBell,
                r#note: 12,
            });
        }
        if state_id == 1064 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#note: 16,
                r#powered: false,
            });
        }
        if state_id == 1188 {
            return Some(NoteBlock {
                r#note: 3,
                r#powered: false,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 1354 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#note: 11,
                r#powered: false,
            });
        }
        if state_id == 1515 {
            return Some(NoteBlock {
                r#note: 17,
                r#instrument: Instrument::Creeper,
                r#powered: true,
            });
        }
        if state_id == 1097 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 8,
                r#powered: true,
            });
        }
        if state_id == 1679 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 24,
                r#powered: true,
            });
        }
        if state_id == 1538 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 3,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 671 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Basedrum,
                r#note: 20,
            });
        }
        if state_id == 1024 {
            return Some(NoteBlock {
                r#note: 21,
                r#powered: false,
                r#instrument: Instrument::Chime,
            });
        }
        if state_id == 628 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 23,
                r#powered: false,
            });
        }
        if state_id == 593 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 6,
                r#powered: true,
            });
        }
        if state_id == 987 {
            return Some(NoteBlock {
                r#note: 3,
                r#powered: true,
                r#instrument: Instrument::Chime,
            });
        }
        if state_id == 991 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Chime,
                r#note: 5,
            });
        }
        if state_id == 655 {
            return Some(NoteBlock {
                r#note: 12,
                r#instrument: Instrument::Basedrum,
                r#powered: true,
            });
        }
        if state_id == 727 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 23,
                r#powered: true,
            });
        }
        if state_id == 858 {
            return Some(NoteBlock {
                r#note: 13,
                r#instrument: Instrument::Flute,
                r#powered: false,
            });
        }
        if state_id == 859 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Flute,
                r#note: 14,
            });
        }
        if state_id == 1213 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#powered: true,
                r#note: 16,
            });
        }
        if state_id == 854 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Flute,
                r#note: 11,
            });
        }
        if state_id == 1238 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 3,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 1230 {
            return Some(NoteBlock {
                r#note: 24,
                r#powered: false,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 1379 {
            return Some(NoteBlock {
                r#note: 24,
                r#powered: true,
                r#instrument: Instrument::Pling,
            });
        }
        if state_id == 1289 {
            return Some(NoteBlock {
                r#note: 4,
                r#powered: true,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 1219 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 19,
                r#instrument: Instrument::Didgeridoo,
            });
        }
        if state_id == 648 {
            return Some(NoteBlock {
                r#note: 8,
                r#instrument: Instrument::Basedrum,
                r#powered: false,
            });
        }
        if state_id == 939 {
            return Some(NoteBlock {
                r#note: 4,
                r#powered: true,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 968 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#powered: false,
                r#note: 18,
            });
        }
        if state_id == 936 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#powered: false,
                r#note: 2,
            });
        }
        if state_id == 1607 {
            return Some(NoteBlock {
                r#note: 13,
                r#powered: true,
                r#instrument: Instrument::WitherSkeleton,
            });
        }
        if state_id == 923 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bell,
                r#powered: true,
                r#note: 21,
            });
        }
        if state_id == 1014 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#note: 16,
                r#powered: false,
            });
        }
        if state_id == 1356 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#powered: false,
                r#note: 12,
            });
        }
        if state_id == 1169 {
            return Some(NoteBlock {
                r#note: 19,
                r#instrument: Instrument::CowBell,
                r#powered: true,
            });
        }
        if state_id == 1547 {
            return Some(NoteBlock {
                r#note: 8,
                r#powered: true,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 1686 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::CustomHead,
                r#note: 2,
            });
        }
        if state_id == 601 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Harp,
                r#note: 10,
            });
        }
        if state_id == 1674 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Piglin,
                r#note: 21,
            });
        }
        if state_id == 1526 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#powered: false,
                r#note: 22,
            });
        }
        if state_id == 1487 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 3,
                r#powered: true,
            });
        }
        if state_id == 1273 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bit,
                r#note: 21,
            });
        }
        if state_id == 1047 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#powered: true,
                r#note: 8,
            });
        }
        if state_id == 1421 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Zombie,
                r#note: 20,
            });
        }
        if state_id == 1664 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 16,
                r#powered: false,
            });
        }
        if state_id == 1713 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 16,
                r#instrument: Instrument::CustomHead,
            });
        }
        if state_id == 1099 {
            return Some(NoteBlock {
                r#note: 9,
                r#powered: true,
                r#instrument: Instrument::IronXylophone,
            });
        }
        if state_id == 635 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 2,
                r#instrument: Instrument::Basedrum,
            });
        }
        if state_id == 977 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#powered: true,
                r#note: 23,
            });
        }
        if state_id == 1608 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::WitherSkeleton,
                r#note: 13,
            });
        }
        if state_id == 1259 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Bit,
                r#note: 14,
            });
        }
        if state_id == 1233 {
            return Some(NoteBlock {
                r#note: 1,
                r#powered: true,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 1328 {
            return Some(NoteBlock {
                r#note: 23,
                r#powered: false,
                r#instrument: Instrument::Banjo,
            });
        }
        if state_id == 1646 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#powered: false,
                r#note: 7,
            });
        }
        if state_id == 1391 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 5,
                r#instrument: Instrument::Zombie,
            });
        }
        if state_id == 956 {
            return Some(NoteBlock {
                r#note: 12,
                r#instrument: Instrument::Guitar,
                r#powered: false,
            });
        }
        if state_id == 686 {
            return Some(NoteBlock {
                r#note: 2,
                r#powered: false,
                r#instrument: Instrument::Snare,
            });
        }
        if state_id == 994 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Chime,
                r#note: 6,
            });
        }
        if state_id == 1240 {
            return Some(NoteBlock {
                r#note: 4,
                r#powered: false,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 1140 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#note: 4,
                r#powered: false,
            });
        }
        if state_id == 736 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#powered: false,
                r#note: 2,
            });
        }
        if state_id == 1090 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 4,
                r#powered: false,
            });
        }
        if state_id == 1637 {
            return Some(NoteBlock {
                r#instrument: Instrument::Piglin,
                r#note: 3,
                r#powered: true,
            });
        }
        if state_id == 1332 {
            return Some(NoteBlock {
                r#instrument: Instrument::Pling,
                r#note: 0,
                r#powered: false,
            });
        }
        if state_id == 1518 {
            return Some(NoteBlock {
                r#powered: false,
                r#note: 18,
                r#instrument: Instrument::Creeper,
            });
        }
        if state_id == 761 {
            return Some(NoteBlock {
                r#instrument: Instrument::Hat,
                r#powered: true,
                r#note: 15,
            });
        }
        if state_id == 1227 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#note: 23,
                r#powered: true,
            });
        }
        if state_id == 1231 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#note: 0,
                r#powered: true,
            });
        }
        if state_id == 1399 {
            return Some(NoteBlock {
                r#note: 9,
                r#powered: true,
                r#instrument: Instrument::Zombie,
            });
        }
        if state_id == 1702 {
            return Some(NoteBlock {
                r#instrument: Instrument::CustomHead,
                r#powered: false,
                r#note: 10,
            });
        }
        if state_id == 1094 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::IronXylophone,
                r#note: 6,
            });
        }
        if state_id == 1324 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 21,
                r#powered: false,
            });
        }
        if state_id == 1139 {
            return Some(NoteBlock {
                r#note: 4,
                r#powered: true,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 1269 {
            return Some(NoteBlock {
                r#note: 19,
                r#powered: true,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 662 {
            return Some(NoteBlock {
                r#instrument: Instrument::Basedrum,
                r#powered: false,
                r#note: 15,
            });
        }
        if state_id == 974 {
            return Some(NoteBlock {
                r#note: 21,
                r#powered: false,
                r#instrument: Instrument::Guitar,
            });
        }
        if state_id == 1133 {
            return Some(NoteBlock {
                r#note: 1,
                r#powered: true,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 1489 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#powered: true,
                r#note: 4,
            });
        }
        if state_id == 882 {
            return Some(NoteBlock {
                r#note: 0,
                r#powered: false,
                r#instrument: Instrument::Bell,
            });
        }
        if state_id == 1308 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 13,
                r#powered: false,
            });
        }
        if state_id == 1568 {
            return Some(NoteBlock {
                r#note: 18,
                r#powered: false,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 1464 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Skeleton,
                r#note: 16,
            });
        }
        if state_id == 1521 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 20,
                r#powered: true,
            });
        }
        if state_id == 1221 {
            return Some(NoteBlock {
                r#note: 20,
                r#instrument: Instrument::Didgeridoo,
                r#powered: true,
            });
        }
        if state_id == 1297 {
            return Some(NoteBlock {
                r#note: 8,
                r#instrument: Instrument::Banjo,
                r#powered: true,
            });
        }
        if state_id == 633 {
            return Some(NoteBlock {
                r#note: 1,
                r#instrument: Instrument::Basedrum,
                r#powered: true,
            });
        }
        if state_id == 757 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 13,
                r#instrument: Instrument::Hat,
            });
        }
        if state_id == 1091 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 5,
                r#powered: true,
            });
        }
        if state_id == 1452 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Skeleton,
                r#note: 10,
            });
        }
        if state_id == 1168 {
            return Some(NoteBlock {
                r#note: 18,
                r#powered: false,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 724 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 21,
                r#powered: false,
            });
        }
        if state_id == 1010 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Chime,
                r#note: 14,
            });
        }
        if state_id == 689 {
            return Some(NoteBlock {
                r#note: 4,
                r#instrument: Instrument::Snare,
                r#powered: true,
            });
        }
        if state_id == 1511 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Creeper,
                r#note: 15,
            });
        }
        if state_id == 829 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 24,
                r#powered: true,
            });
        }
        if state_id == 1176 {
            return Some(NoteBlock {
                r#instrument: Instrument::CowBell,
                r#note: 22,
                r#powered: false,
            });
        }
        if state_id == 1433 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 1,
                r#instrument: Instrument::Skeleton,
            });
        }
        if state_id == 1110 {
            return Some(NoteBlock {
                r#instrument: Instrument::IronXylophone,
                r#note: 14,
                r#powered: false,
            });
        }
        if state_id == 1505 {
            return Some(NoteBlock {
                r#note: 12,
                r#powered: true,
                r#instrument: Instrument::Creeper,
            });
        }
        if state_id == 1171 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 20,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 611 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#note: 15,
                r#powered: true,
            });
        }
        if state_id == 838 {
            return Some(NoteBlock {
                r#instrument: Instrument::Flute,
                r#powered: false,
                r#note: 3,
            });
        }
        if state_id == 960 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#note: 14,
                r#powered: false,
            });
        }
        if state_id == 1190 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#powered: false,
                r#note: 4,
            });
        }
        if state_id == 814 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 16,
                r#powered: false,
            });
        }
        if state_id == 1208 {
            return Some(NoteBlock {
                r#instrument: Instrument::Didgeridoo,
                r#powered: false,
                r#note: 13,
            });
        }
        if state_id == 1256 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bit,
                r#powered: false,
                r#note: 12,
            });
        }
        if state_id == 1445 {
            return Some(NoteBlock {
                r#note: 7,
                r#powered: true,
                r#instrument: Instrument::Skeleton,
            });
        }
        if state_id == 1085 {
            return Some(NoteBlock {
                r#note: 2,
                r#powered: true,
                r#instrument: Instrument::IronXylophone,
            });
        }
        if state_id == 821 {
            return Some(NoteBlock {
                r#instrument: Instrument::Bass,
                r#note: 20,
                r#powered: true,
            });
        }
        if state_id == 1494 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 6,
                r#powered: false,
            });
        }
        if state_id == 1535 {
            return Some(NoteBlock {
                r#note: 2,
                r#powered: true,
                r#instrument: Instrument::Dragon,
            });
        }
        if state_id == 692 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 5,
                r#powered: false,
            });
        }
        if state_id == 1299 {
            return Some(NoteBlock {
                r#instrument: Instrument::Banjo,
                r#note: 9,
                r#powered: true,
            });
        }
        if state_id == 1653 {
            return Some(NoteBlock {
                r#note: 11,
                r#powered: true,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 1697 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::CustomHead,
                r#note: 8,
            });
        }
        if state_id == 1252 {
            return Some(NoteBlock {
                r#note: 10,
                r#instrument: Instrument::Bit,
                r#powered: false,
            });
        }
        if state_id == 1395 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Zombie,
                r#note: 7,
            });
        }
        if state_id == 1009 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#note: 14,
                r#powered: true,
            });
        }
        if state_id == 1149 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 9,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 1546 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#note: 7,
                r#powered: false,
            });
        }
        if state_id == 953 {
            return Some(NoteBlock {
                r#instrument: Instrument::Guitar,
                r#note: 11,
                r#powered: true,
            });
        }
        if state_id == 626 {
            return Some(NoteBlock {
                r#instrument: Instrument::Harp,
                r#powered: false,
                r#note: 22,
            });
        }
        if state_id == 1566 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#powered: false,
                r#note: 17,
            });
        }
        if state_id == 1499 {
            return Some(NoteBlock {
                r#instrument: Instrument::Creeper,
                r#note: 9,
                r#powered: true,
            });
        }
        if state_id == 831 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Flute,
                r#note: 0,
            });
        }
        if state_id == 1062 {
            return Some(NoteBlock {
                r#note: 15,
                r#powered: false,
                r#instrument: Instrument::Xylophone,
            });
        }
        if state_id == 1131 {
            return Some(NoteBlock {
                r#note: 0,
                r#powered: true,
                r#instrument: Instrument::CowBell,
            });
        }
        if state_id == 1192 {
            return Some(NoteBlock {
                r#powered: false,
                r#instrument: Instrument::Didgeridoo,
                r#note: 5,
            });
        }
        if state_id == 1570 {
            return Some(NoteBlock {
                r#instrument: Instrument::Dragon,
                r#note: 19,
                r#powered: false,
            });
        }
        if state_id == 1001 {
            return Some(NoteBlock {
                r#instrument: Instrument::Chime,
                r#note: 10,
                r#powered: true,
            });
        }
        if state_id == 1616 {
            return Some(NoteBlock {
                r#instrument: Instrument::WitherSkeleton,
                r#note: 17,
                r#powered: false,
            });
        }
        if state_id == 1659 {
            return Some(NoteBlock {
                r#powered: true,
                r#note: 14,
                r#instrument: Instrument::Piglin,
            });
        }
        if state_id == 1264 {
            return Some(NoteBlock {
                r#note: 16,
                r#powered: false,
                r#instrument: Instrument::Bit,
            });
        }
        if state_id == 1623 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::WitherSkeleton,
                r#note: 21,
            });
        }
        if state_id == 1036 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#powered: false,
                r#note: 2,
            });
        }
        if state_id == 1729 {
            return Some(NoteBlock {
                r#note: 24,
                r#instrument: Instrument::CustomHead,
                r#powered: true,
            });
        }
        if state_id == 1189 {
            return Some(NoteBlock {
                r#powered: true,
                r#instrument: Instrument::Didgeridoo,
                r#note: 4,
            });
        }
        if state_id == 698 {
            return Some(NoteBlock {
                r#instrument: Instrument::Snare,
                r#note: 8,
                r#powered: false,
            });
        }
        if state_id == 1041 {
            return Some(NoteBlock {
                r#instrument: Instrument::Xylophone,
                r#powered: true,
                r#note: 5,
            });
        }
        if state_id == 915 {
            return Some(NoteBlock {
                r#note: 17,
                r#instrument: Instrument::Bell,
                r#powered: true,
            });
        }
        return None;
    }
}

