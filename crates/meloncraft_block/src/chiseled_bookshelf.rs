use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChiseledBookshelf {
    pub slot_4_occupied: bool,
    pub slot_5_occupied: bool,
    pub r#facing: Facing,
    pub slot_1_occupied: bool,
    pub slot_0_occupied: bool,
    pub slot_2_occupied: bool,
    pub slot_3_occupied: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for ChiseledBookshelf {
    fn to_id(self) -> i32 {
        if block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::West { return 2315; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == false { return 2224; }
        if block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == true { return 2160; }
        if block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == false { return 2286; }
        if block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == false { return 2178; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::West { return 2330; }
        if block_state.r#facing == Facing::North && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false { return 2190; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_0_occupied == false { return 2245; }
        if block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == true { return 2366; }
        if block_state.r#facing == Facing::South && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == true { return 2222; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false { return 2244; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == true { return 2194; }
        if block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#facing == Facing::North { return 2188; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_5_occupied == true { return 2389; }
        if block_state.r#facing == Facing::North && block_state.r#slot_4_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_5_occupied == false { return 2152; }
        if block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false { return 2204; }
        if block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false { return 2341; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false { return 2381; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == true { return 2351; }
        if block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == true { return 2358; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_3_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true { return 2177; }
        if block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == false { return 2291; }
        if block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == true { return 2151; }
        if block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_1_occupied == false { return 2297; }
        if block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == true { return 2234; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == true { return 2338; }
        if block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == true { return 2343; }
        if block_state.r#facing == Facing::East && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == true { return 2344; }
        if block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false { return 2398; }
        if block_state.r#slot_5_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_2_occupied == true { return 2258; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_0_occupied == false { return 2175; }
        if block_state.r#slot_3_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == true { return 2148; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == true { return 2225; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::West { return 2295; }
        if block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_0_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_1_occupied == true { return 2156; }
        if block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == false { return 2202; }
        if block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == false { return 2384; }
        if block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#facing == Facing::North { return 2182; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == false { return 2300; }
        if block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_4_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == false { return 2307; }
        if block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == false { return 2288; }
        if block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true { return 2215; }
        if block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_4_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_5_occupied == false { return 2192; }
        if block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == true { return 2391; }
        if block_state.r#facing == Facing::North && block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == false { return 2154; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false { return 2168; }
        if block_state.r#facing == Facing::North && block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == false { return 2191; }
        if block_state.r#facing == Facing::West && block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true { return 2282; }
        if block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_2_occupied == true && block_state.r#slot_5_occupied == false { return 2292; }
        if block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == false { return 2350; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == true { return 2387; }
        if block_state.r#slot_5_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == true { return 2276; }
        if block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == false { return 2163; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == true { return 2320; }
        if block_state.r#facing == Facing::South && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false { return 2250; }
        if block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true { return 2264; }
        if block_state.r#facing == Facing::West && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false { return 2278; }
        if block_state.r#facing == Facing::North && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == true { return 2155; }
        if block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_4_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == true { return 2153; }
        if block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false { return 2348; }
        if block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == false { return 2186; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::South { return 2229; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == true { return 2335; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_4_occupied == true { return 2355; }
        if block_state.r#facing == Facing::South && block_state.r#slot_2_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == false { return 2246; }
        if block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_1_occupied == true { return 2242; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::East { return 2354; }
        if block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == true { return 2281; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_0_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false { return 2325; }
        if block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false { return 2372; }
        if block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == false { return 2365; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == false { return 2238; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == true { return 2337; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == false { return 2211; }
        if block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false { return 2294; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true { return 2362; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_1_occupied == false { return 2386; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::North { return 2162; }
        if block_state.r#slot_2_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false { return 2322; }
        if block_state.r#slot_1_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::South { return 2221; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == true { return 2239; }
        if block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false { return 2328; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#facing == Facing::East { return 2383; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true { return 2375; }
        if block_state.r#facing == Facing::South && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true { return 2217; }
        if block_state.r#facing == Facing::South && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false { return 2268; }
        if block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false { return 2219; }
        if block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_4_occupied == true { return 2267; }
        if block_state.r#facing == Facing::East && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false { return 2364; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == true { return 2209; }
        if block_state.r#facing == Facing::South && block_state.r#slot_3_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_0_occupied == false { return 2269; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false { return 2263; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::North { return 2166; }
        if block_state.r#slot_5_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_3_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == true { return 2214; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false { return 2289; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true { return 2305; }
        if block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == true { return 2220; }
        if block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == true { return 2280; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true { return 2373; }
        if block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == false { return 2394; }
        if block_state.r#facing == Facing::West && block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == false { return 2333; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == true { return 2336; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == false { return 2349; }
        if block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true { return 2323; }
        if block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == true { return 2299; }
        if block_state.r#slot_4_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_0_occupied == true { return 2271; }
        if block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == false { return 2382; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::North { return 2171; }
        if block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::South { return 2248; }
        if block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == false { return 2352; }
        if block_state.r#slot_5_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false { return 2356; }
        if block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true { return 2251; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == true { return 2216; }
        if block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_1_occupied == false { return 2392; }
        if block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == true { return 2174; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == true { return 2283; }
        if block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == true { return 2324; }
        if block_state.r#slot_2_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true { return 2181; }
        if block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_5_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == true { return 2196; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::West { return 2326; }
        if block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == false { return 2198; }
        if block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == false { return 2318; }
        if block_state.r#slot_0_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_4_occupied == false { return 2306; }
        if block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true { return 2393; }
        if block_state.r#facing == Facing::South && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == false { return 2252; }
        if block_state.r#facing == Facing::West && block_state.r#slot_1_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true { return 2275; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::West { return 2277; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true { return 2213; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true { return 2314; }
        if block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_3_occupied == true { return 2287; }
        if block_state.r#slot_2_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_5_occupied == false { return 2388; }
        if block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_4_occupied == true { return 2231; }
        if block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_3_occupied == true && block_state.r#slot_0_occupied == false { return 2266; }
        if block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == true { return 2227; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == false { return 2187; }
        if block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true { return 2339; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true { return 2345; }
        if block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true { return 2183; }
        if block_state.r#slot_3_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == true { return 2309; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_1_occupied == true { return 2279; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_1_occupied == true { return 2284; }
        if block_state.r#facing == Facing::North && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false { return 2158; }
        if block_state.r#facing == Facing::West && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == true { return 2319; }
        if block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false { return 2189; }
        if block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true { return 2285; }
        if block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_2_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == true { return 2304; }
        if block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == false { return 2146; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == true { return 2243; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == true { return 2159; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == true { return 2346; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::West { return 2313; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::North { return 2164; }
        if block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_1_occupied == false { return 2396; }
        if block_state.r#facing == Facing::West && block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == false { return 2332; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == false { return 2293; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == true { return 2223; }
        if block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == true { return 2327; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_3_occupied == false { return 2157; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == true { return 2274; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_4_occupied == true { return 2368; }
        if block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_5_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_1_occupied == true { return 2254; }
        if block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_5_occupied == true { return 2367; }
        if block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == false { return 2390; }
        if block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_5_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == false { return 2205; }
        if block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::North { return 2167; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_0_occupied == false && block_state.r#slot_3_occupied == true { return 2329; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::South { return 2226; }
        if block_state.r#facing == Facing::South && block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false { return 2262; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == false { return 2170; }
        if block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true { return 2312; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_4_occupied == true { return 2340; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false { return 2237; }
        if block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true { return 2385; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == false { return 2303; }
        if block_state.r#facing == Facing::East && block_state.r#slot_4_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == true { return 2347; }
        if block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == false { return 2232; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#facing == Facing::South { return 2241; }
        if block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_2_occupied == true { return 2210; }
        if block_state.r#facing == Facing::South && block_state.r#slot_3_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_5_occupied == false { return 2270; }
        if block_state.r#facing == Facing::West && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == true { return 2302; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#facing == Facing::North { return 2195; }
        if block_state.r#facing == Facing::North && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_3_occupied == true { return 2185; }
        if block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == true { return 2255; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == false { return 2265; }
        if block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == false { return 2353; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::South { return 2235; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == false { return 2261; }
        if block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == true { return 2212; }
        if block_state.r#facing == Facing::North && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == false { return 2193; }
        if block_state.r#facing == Facing::East && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == false { return 2360; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == false { return 2395; }
        if block_state.r#facing == Facing::North && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_5_occupied == true { return 2149; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_0_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true { return 2179; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == true { return 2371; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_0_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == false { return 2201; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::East { return 2379; }
        if block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_0_occupied == false { return 2380; }
        if block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true { return 2253; }
        if block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false { return 2397; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == true { return 2233; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == true { return 2311; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::North { return 2144; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_0_occupied == true { return 2161; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_5_occupied == false { return 2236; }
        if block_state.r#facing == Facing::North && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == false { return 2165; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == true { return 2359; }
        if block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == true { return 2249; }
        if block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::West { return 2331; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == true { return 2301; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == false { return 2290; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::East { return 2342; }
        if block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::East { return 2378; }
        if block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false { return 2218; }
        if block_state.r#slot_4_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == false { return 2296; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true { return 2176; }
        if block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == true { return 2308; }
        if block_state.r#facing == Facing::West && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == true { return 2310; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true { return 2321; }
        if block_state.r#facing == Facing::North && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == true { return 2145; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_4_occupied == true { return 2207; }
        if block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_1_occupied == true { return 2376; }
        if block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true { return 2363; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_2_occupied == true { return 2208; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == false { return 2173; }
        if block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::North { return 2199; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == true { return 2257; }
        if block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::North { return 2180; }
        if block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::West { return 2334; }
        if block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == true { return 2272; }
        if block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == false { return 2369; }
        if block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false { return 2184; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false { return 2228; }
        if block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_2_occupied == true { return 2374; }
        if block_state.r#facing == Facing::North && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false { return 2169; }
        if block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == false { return 2150; }
        if block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == false { return 2206; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == true { return 2143; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_5_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == true { return 2230; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_2_occupied == true { return 2256; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_2_occupied == false && block_state.r#slot_1_occupied == true && block_state.r#slot_4_occupied == false { return 2377; }
        if block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_2_occupied == true && block_state.r#slot_3_occupied == false { return 2197; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_1_occupied == true && block_state.r#slot_3_occupied == true { return 2240; }
        if block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true { return 2203; }
        if block_state.r#facing == Facing::South && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_4_occupied == true && block_state.r#slot_3_occupied == true { return 2247; }
        if block_state.r#slot_1_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_4_occupied == false { return 2370; }
        if block_state.r#facing == Facing::East && block_state.r#slot_0_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_5_occupied == true { return 2361; }
        if block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#facing == Facing::West && block_state.r#slot_3_occupied == false { return 2317; }
        if block_state.r#slot_4_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true { return 2200; }
        if block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#facing == Facing::East && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == false { return 2357; }
        if block_state.r#slot_4_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::West && block_state.r#slot_0_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == true && block_state.r#slot_5_occupied == false { return 2298; }
        if block_state.r#facing == Facing::West && block_state.r#slot_1_occupied == true && block_state.r#slot_2_occupied == false && block_state.r#slot_3_occupied == false && block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_4_occupied == true { return 2316; }
        if block_state.r#slot_3_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#slot_0_occupied == true && block_state.r#slot_5_occupied == false && block_state.r#facing == Facing::North && block_state.r#slot_2_occupied == false { return 2172; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_4_occupied == false && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::West { return 2273; }
        if block_state.r#slot_5_occupied == false && block_state.r#slot_0_occupied == false && block_state.r#slot_1_occupied == false && block_state.r#slot_4_occupied == true && block_state.r#facing == Facing::South && block_state.r#slot_3_occupied == false && block_state.r#slot_2_occupied == true { return 2260; }
        if block_state.r#slot_2_occupied == true && block_state.r#slot_1_occupied == false && block_state.r#facing == Facing::South && block_state.r#slot_4_occupied == true && block_state.r#slot_5_occupied == true && block_state.r#slot_0_occupied == false && block_state.r#slot_3_occupied == false { return 2259; }
        if block_state.r#slot_5_occupied == true && block_state.r#slot_1_occupied == true && block_state.r#facing == Facing::North && block_state.r#slot_4_occupied == true && block_state.r#slot_2_occupied == true && block_state.r#slot_0_occupied == true && block_state.r#slot_3_occupied == false { return 2147; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2315 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#slot_0_occupied: false,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#slot_4_occupied: true,
                r#slot_3_occupied: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2224 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_4_occupied: true,
                r#slot_2_occupied: true,
                r#facing: Facing::South,
                r#slot_0_occupied: true,
                r#slot_1_occupied: false,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2160 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#facing: Facing::North,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_1_occupied: false,
                r#slot_2_occupied: true,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2286 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: true,
                r#slot_2_occupied: false,
                r#slot_4_occupied: false,
                r#slot_0_occupied: true,
                r#facing: Facing::West,
                r#slot_5_occupied: false,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2178 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#slot_4_occupied: false,
                r#slot_1_occupied: true,
                r#facing: Facing::North,
                r#slot_2_occupied: true,
                r#slot_3_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2330 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_0_occupied: false,
                r#slot_5_occupied: false,
                r#slot_4_occupied: false,
                r#slot_1_occupied: false,
                r#slot_2_occupied: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2190 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::North,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_2_occupied: false,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
            });
        }
        if state_id == 2245 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_2_occupied: true,
                r#facing: Facing::South,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_0_occupied: false,
            });
        }
        if state_id == 2366 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#facing: Facing::East,
                r#slot_1_occupied: false,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2222 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::South,
                r#slot_3_occupied: false,
                r#slot_5_occupied: false,
                r#slot_4_occupied: false,
                r#slot_2_occupied: false,
                r#slot_1_occupied: true,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2244 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_1_occupied: true,
                r#facing: Facing::South,
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
            });
        }
        if state_id == 2194 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_0_occupied: false,
                r#facing: Facing::North,
                r#slot_5_occupied: false,
                r#slot_3_occupied: true,
                r#slot_1_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2188 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#slot_3_occupied: false,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#slot_2_occupied: false,
                r#slot_4_occupied: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2389 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_0_occupied: false,
                r#slot_1_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#facing: Facing::East,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2152 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::North,
                r#slot_4_occupied: true,
                r#slot_1_occupied: true,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
                r#slot_0_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2204 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#slot_1_occupied: false,
                r#facing: Facing::North,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2341 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#facing: Facing::East,
                r#slot_2_occupied: true,
                r#slot_0_occupied: true,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2381 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#slot_5_occupied: true,
                r#facing: Facing::East,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2351 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_1_occupied: false,
                r#facing: Facing::East,
                r#slot_2_occupied: true,
                r#slot_4_occupied: true,
                r#slot_0_occupied: true,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2358 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#facing: Facing::East,
                r#slot_5_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_1_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2177 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#facing: Facing::North,
                r#slot_3_occupied: true,
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2291 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#slot_5_occupied: true,
                r#slot_0_occupied: true,
                r#facing: Facing::West,
                r#slot_2_occupied: true,
                r#slot_4_occupied: true,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2151 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#slot_3_occupied: true,
                r#facing: Facing::North,
                r#slot_0_occupied: true,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2297 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
                r#slot_5_occupied: true,
                r#slot_0_occupied: true,
                r#slot_4_occupied: false,
                r#facing: Facing::West,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2234 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
                r#facing: Facing::South,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2338 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_1_occupied: true,
                r#slot_0_occupied: true,
                r#slot_5_occupied: false,
                r#slot_3_occupied: true,
                r#facing: Facing::East,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2343 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#slot_2_occupied: false,
                r#facing: Facing::East,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2344 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_3_occupied: true,
                r#slot_4_occupied: true,
                r#slot_2_occupied: false,
                r#slot_5_occupied: false,
                r#slot_1_occupied: true,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2398 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#facing: Facing::East,
                r#slot_0_occupied: false,
                r#slot_1_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2258 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#slot_4_occupied: false,
                r#slot_1_occupied: false,
                r#facing: Facing::South,
                r#slot_0_occupied: false,
                r#slot_3_occupied: true,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2175 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#facing: Facing::North,
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
                r#slot_0_occupied: false,
            });
        }
        if state_id == 2148 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: false,
                r#slot_1_occupied: true,
                r#slot_2_occupied: true,
                r#slot_4_occupied: true,
                r#facing: Facing::North,
                r#slot_5_occupied: false,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2225 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#facing: Facing::South,
                r#slot_0_occupied: true,
                r#slot_1_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2295 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#slot_0_occupied: true,
                r#slot_2_occupied: false,
                r#slot_1_occupied: false,
                r#slot_3_occupied: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2156 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#facing: Facing::North,
                r#slot_0_occupied: true,
                r#slot_5_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2202 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#facing: Facing::North,
                r#slot_1_occupied: false,
                r#slot_2_occupied: false,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2384 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_2_occupied: true,
                r#slot_0_occupied: false,
                r#slot_4_occupied: true,
                r#slot_3_occupied: true,
                r#slot_1_occupied: false,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2182 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: false,
                r#slot_5_occupied: false,
                r#slot_1_occupied: true,
                r#slot_4_occupied: false,
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2300 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#facing: Facing::West,
                r#slot_1_occupied: false,
                r#slot_0_occupied: true,
                r#slot_3_occupied: false,
                r#slot_2_occupied: false,
            });
        }
        if state_id == 2307 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: false,
                r#slot_5_occupied: true,
                r#facing: Facing::West,
                r#slot_4_occupied: true,
                r#slot_1_occupied: true,
                r#slot_2_occupied: true,
                r#slot_0_occupied: false,
            });
        }
        if state_id == 2288 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#slot_2_occupied: true,
                r#slot_4_occupied: true,
                r#slot_0_occupied: true,
                r#facing: Facing::West,
                r#slot_3_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2215 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#facing: Facing::South,
                r#slot_0_occupied: true,
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2192 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#slot_1_occupied: false,
                r#slot_3_occupied: true,
                r#facing: Facing::North,
                r#slot_4_occupied: true,
                r#slot_2_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2391 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#slot_1_occupied: false,
                r#facing: Facing::East,
                r#slot_3_occupied: true,
                r#slot_5_occupied: true,
                r#slot_0_occupied: false,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2154 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::North,
                r#slot_0_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_3_occupied: true,
                r#slot_1_occupied: true,
                r#slot_2_occupied: false,
            });
        }
        if state_id == 2168 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_2_occupied: false,
                r#slot_0_occupied: true,
                r#facing: Facing::North,
                r#slot_1_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2191 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::North,
                r#slot_5_occupied: true,
                r#slot_2_occupied: true,
                r#slot_3_occupied: true,
                r#slot_1_occupied: false,
                r#slot_4_occupied: true,
                r#slot_0_occupied: false,
            });
        }
        if state_id == 2282 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::West,
                r#slot_0_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_2_occupied: false,
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2292 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#slot_0_occupied: true,
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
                r#facing: Facing::West,
                r#slot_2_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2350 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#facing: Facing::East,
                r#slot_0_occupied: true,
                r#slot_1_occupied: true,
                r#slot_2_occupied: false,
            });
        }
        if state_id == 2387 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_0_occupied: false,
                r#slot_4_occupied: true,
                r#slot_3_occupied: false,
                r#facing: Facing::East,
                r#slot_1_occupied: false,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2276 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#slot_4_occupied: true,
                r#facing: Facing::West,
                r#slot_3_occupied: false,
                r#slot_2_occupied: true,
                r#slot_1_occupied: true,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2163 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#facing: Facing::North,
                r#slot_4_occupied: true,
                r#slot_3_occupied: false,
                r#slot_2_occupied: true,
                r#slot_0_occupied: true,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2320 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_5_occupied: false,
                r#slot_3_occupied: true,
                r#facing: Facing::West,
                r#slot_0_occupied: false,
                r#slot_1_occupied: false,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2250 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::South,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2264 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#facing: Facing::South,
                r#slot_0_occupied: false,
                r#slot_1_occupied: false,
                r#slot_4_occupied: true,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2278 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::West,
                r#slot_0_occupied: true,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
                r#slot_2_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2155 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::North,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2153 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#facing: Facing::North,
                r#slot_4_occupied: false,
                r#slot_2_occupied: false,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2348 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#facing: Facing::East,
                r#slot_0_occupied: true,
                r#slot_1_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2186 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
                r#facing: Facing::North,
                r#slot_5_occupied: false,
                r#slot_2_occupied: false,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2229 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_5_occupied: true,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_2_occupied: true,
                r#slot_1_occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2335 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_1_occupied: true,
                r#slot_2_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#facing: Facing::East,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2355 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_1_occupied: false,
                r#slot_0_occupied: true,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
                r#facing: Facing::East,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2246 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::South,
                r#slot_2_occupied: true,
                r#slot_5_occupied: false,
                r#slot_3_occupied: false,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2242 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_3_occupied: true,
                r#facing: Facing::South,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2354 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_1_occupied: false,
                r#slot_2_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_3_occupied: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2281 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#facing: Facing::West,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_2_occupied: false,
                r#slot_1_occupied: true,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2325 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_1_occupied: false,
                r#facing: Facing::West,
                r#slot_0_occupied: false,
                r#slot_5_occupied: true,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2372 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#facing: Facing::East,
                r#slot_4_occupied: true,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2365 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: false,
                r#slot_5_occupied: true,
                r#slot_4_occupied: false,
                r#facing: Facing::East,
                r#slot_0_occupied: true,
                r#slot_2_occupied: false,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2238 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#facing: Facing::South,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2337 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_1_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_3_occupied: true,
                r#facing: Facing::East,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2211 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#facing: Facing::South,
                r#slot_2_occupied: true,
                r#slot_0_occupied: true,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2294 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#facing: Facing::West,
                r#slot_2_occupied: true,
                r#slot_1_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2362 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_1_occupied: false,
                r#facing: Facing::East,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2386 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_0_occupied: false,
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#facing: Facing::East,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2162 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_1_occupied: false,
                r#slot_0_occupied: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2322 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#facing: Facing::West,
                r#slot_4_occupied: false,
                r#slot_1_occupied: false,
                r#slot_3_occupied: true,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
            });
        }
        if state_id == 2221 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: true,
                r#slot_5_occupied: true,
                r#slot_4_occupied: false,
                r#slot_3_occupied: false,
                r#slot_0_occupied: true,
                r#slot_2_occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2239 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_2_occupied: true,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#facing: Facing::South,
                r#slot_4_occupied: true,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2328 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#slot_2_occupied: false,
                r#slot_0_occupied: false,
                r#slot_3_occupied: true,
                r#facing: Facing::West,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2383 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_0_occupied: false,
                r#slot_3_occupied: true,
                r#slot_1_occupied: false,
                r#slot_5_occupied: true,
                r#slot_4_occupied: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2375 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_1_occupied: true,
                r#slot_0_occupied: false,
                r#facing: Facing::East,
                r#slot_2_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2217 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::South,
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
                r#slot_2_occupied: false,
                r#slot_0_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2268 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::South,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
                r#slot_1_occupied: false,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2219 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#slot_4_occupied: true,
                r#facing: Facing::South,
                r#slot_5_occupied: true,
                r#slot_0_occupied: true,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2267 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: false,
                r#slot_2_occupied: false,
                r#slot_5_occupied: true,
                r#slot_1_occupied: false,
                r#slot_0_occupied: false,
                r#facing: Facing::South,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2364 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_1_occupied: false,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_0_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2209 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_0_occupied: true,
                r#slot_4_occupied: false,
                r#facing: Facing::South,
                r#slot_1_occupied: true,
                r#slot_2_occupied: true,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2269 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::South,
                r#slot_3_occupied: false,
                r#slot_1_occupied: false,
                r#slot_2_occupied: false,
                r#slot_5_occupied: true,
                r#slot_4_occupied: false,
                r#slot_0_occupied: false,
            });
        }
        if state_id == 2263 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_4_occupied: true,
                r#facing: Facing::South,
                r#slot_5_occupied: true,
                r#slot_1_occupied: false,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
            });
        }
        if state_id == 2166 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
                r#slot_0_occupied: true,
                r#slot_4_occupied: false,
                r#slot_1_occupied: false,
                r#slot_5_occupied: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2214 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#slot_4_occupied: false,
                r#facing: Facing::South,
                r#slot_3_occupied: false,
                r#slot_1_occupied: true,
                r#slot_0_occupied: true,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2289 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_0_occupied: true,
                r#slot_1_occupied: false,
                r#slot_5_occupied: true,
                r#facing: Facing::West,
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2305 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_2_occupied: true,
                r#slot_4_occupied: false,
                r#slot_3_occupied: true,
                r#facing: Facing::West,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2220 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#facing: Facing::South,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2280 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#facing: Facing::West,
                r#slot_3_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_2_occupied: false,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2373 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_0_occupied: false,
                r#facing: Facing::East,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2394 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
                r#slot_0_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2333 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::West,
                r#slot_5_occupied: true,
                r#slot_2_occupied: false,
                r#slot_0_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2336 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_5_occupied: false,
                r#facing: Facing::East,
                r#slot_1_occupied: true,
                r#slot_2_occupied: true,
                r#slot_3_occupied: true,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2349 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#slot_2_occupied: false,
                r#slot_4_occupied: false,
                r#facing: Facing::East,
                r#slot_0_occupied: true,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2323 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#slot_1_occupied: false,
                r#slot_3_occupied: false,
                r#facing: Facing::West,
                r#slot_2_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2299 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: false,
                r#facing: Facing::West,
                r#slot_2_occupied: false,
                r#slot_1_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2271 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#facing: Facing::West,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#slot_2_occupied: true,
                r#slot_3_occupied: true,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2382 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#slot_3_occupied: false,
                r#slot_1_occupied: true,
                r#slot_0_occupied: false,
                r#facing: Facing::East,
                r#slot_2_occupied: false,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2171 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_4_occupied: true,
                r#slot_1_occupied: false,
                r#slot_5_occupied: true,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2248 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2352 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_2_occupied: true,
                r#slot_0_occupied: true,
                r#slot_3_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2356 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#slot_4_occupied: true,
                r#slot_0_occupied: true,
                r#slot_1_occupied: false,
                r#facing: Facing::East,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2251 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::South,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
                r#slot_2_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2216 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_1_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#facing: Facing::South,
                r#slot_2_occupied: false,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2392 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#facing: Facing::East,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
                r#slot_5_occupied: false,
                r#slot_4_occupied: true,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2174 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#facing: Facing::North,
                r#slot_5_occupied: false,
                r#slot_2_occupied: false,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2283 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_3_occupied: false,
                r#slot_5_occupied: true,
                r#slot_2_occupied: false,
                r#facing: Facing::West,
                r#slot_1_occupied: true,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2324 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#slot_3_occupied: false,
                r#facing: Facing::West,
                r#slot_1_occupied: false,
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2181 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#facing: Facing::North,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2196 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#slot_0_occupied: false,
                r#facing: Facing::North,
                r#slot_5_occupied: false,
                r#slot_4_occupied: true,
                r#slot_3_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2326 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_2_occupied: true,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
                r#slot_1_occupied: false,
                r#slot_3_occupied: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2198 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#facing: Facing::North,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2318 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: true,
                r#slot_2_occupied: false,
                r#facing: Facing::West,
                r#slot_3_occupied: false,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2306 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#slot_5_occupied: false,
                r#slot_1_occupied: true,
                r#slot_2_occupied: true,
                r#slot_3_occupied: true,
                r#facing: Facing::West,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2393 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
                r#slot_1_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2252 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::South,
                r#slot_1_occupied: true,
                r#slot_4_occupied: true,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2275 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::West,
                r#slot_1_occupied: true,
                r#slot_5_occupied: true,
                r#slot_0_occupied: true,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2277 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_1_occupied: true,
                r#slot_5_occupied: true,
                r#slot_0_occupied: true,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2213 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_3_occupied: false,
                r#slot_1_occupied: true,
                r#facing: Facing::South,
                r#slot_2_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2314 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#facing: Facing::West,
                r#slot_2_occupied: false,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2287 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#slot_4_occupied: true,
                r#slot_0_occupied: true,
                r#slot_2_occupied: true,
                r#slot_5_occupied: true,
                r#facing: Facing::West,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2388 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#facing: Facing::East,
                r#slot_1_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
                r#slot_0_occupied: false,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2231 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#facing: Facing::South,
                r#slot_1_occupied: false,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
                r#slot_5_occupied: true,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2266 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#slot_1_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#facing: Facing::South,
                r#slot_3_occupied: true,
                r#slot_0_occupied: false,
            });
        }
        if state_id == 2227 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: false,
                r#facing: Facing::South,
                r#slot_2_occupied: true,
                r#slot_1_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2187 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_3_occupied: false,
                r#facing: Facing::North,
                r#slot_2_occupied: false,
                r#slot_1_occupied: true,
                r#slot_4_occupied: true,
                r#slot_0_occupied: false,
            });
        }
        if state_id == 2339 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: true,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
                r#slot_0_occupied: true,
                r#facing: Facing::East,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2345 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_1_occupied: true,
                r#slot_4_occupied: false,
                r#facing: Facing::East,
                r#slot_5_occupied: true,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2183 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#facing: Facing::North,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2309 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: false,
                r#slot_0_occupied: false,
                r#facing: Facing::West,
                r#slot_4_occupied: false,
                r#slot_1_occupied: true,
                r#slot_5_occupied: true,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2279 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_3_occupied: true,
                r#slot_0_occupied: true,
                r#slot_2_occupied: false,
                r#slot_5_occupied: true,
                r#facing: Facing::West,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2284 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_0_occupied: true,
                r#slot_3_occupied: false,
                r#slot_2_occupied: false,
                r#slot_5_occupied: false,
                r#facing: Facing::West,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2158 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::North,
                r#slot_5_occupied: false,
                r#slot_2_occupied: false,
                r#slot_0_occupied: true,
                r#slot_4_occupied: false,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2319 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::West,
                r#slot_1_occupied: false,
                r#slot_3_occupied: true,
                r#slot_5_occupied: true,
                r#slot_2_occupied: true,
                r#slot_0_occupied: false,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2189 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: false,
                r#facing: Facing::North,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
            });
        }
        if state_id == 2285 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#facing: Facing::West,
                r#slot_0_occupied: true,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2304 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#facing: Facing::West,
                r#slot_2_occupied: true,
                r#slot_5_occupied: false,
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2146 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#slot_0_occupied: true,
                r#slot_2_occupied: true,
                r#slot_3_occupied: true,
                r#facing: Facing::North,
                r#slot_1_occupied: true,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2243 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_1_occupied: true,
                r#facing: Facing::South,
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2159 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_3_occupied: true,
                r#slot_5_occupied: true,
                r#slot_1_occupied: false,
                r#facing: Facing::North,
                r#slot_0_occupied: true,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2346 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_2_occupied: false,
                r#facing: Facing::East,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_1_occupied: true,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2313 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2164 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_3_occupied: false,
                r#slot_5_occupied: false,
                r#slot_2_occupied: true,
                r#slot_1_occupied: false,
                r#slot_0_occupied: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2396 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_0_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2332 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::West,
                r#slot_4_occupied: true,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_1_occupied: false,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2293 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_1_occupied: false,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
                r#facing: Facing::West,
                r#slot_0_occupied: true,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2223 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#facing: Facing::South,
                r#slot_3_occupied: true,
                r#slot_1_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2327 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#facing: Facing::West,
                r#slot_3_occupied: true,
                r#slot_4_occupied: true,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2157 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_2_occupied: false,
                r#slot_1_occupied: true,
                r#facing: Facing::North,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2274 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
                r#facing: Facing::West,
                r#slot_5_occupied: false,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2368 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
                r#facing: Facing::East,
                r#slot_0_occupied: false,
                r#slot_5_occupied: false,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2254 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#facing: Facing::South,
                r#slot_5_occupied: false,
                r#slot_4_occupied: false,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2367 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
                r#slot_4_occupied: true,
                r#slot_2_occupied: true,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2390 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
                r#facing: Facing::East,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2205 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#slot_0_occupied: false,
                r#facing: Facing::North,
                r#slot_5_occupied: true,
                r#slot_3_occupied: false,
                r#slot_2_occupied: false,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2167 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#slot_3_occupied: true,
                r#slot_1_occupied: false,
                r#slot_0_occupied: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2329 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_2_occupied: false,
                r#slot_4_occupied: false,
                r#slot_1_occupied: false,
                r#facing: Facing::West,
                r#slot_0_occupied: false,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2226 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_0_occupied: true,
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
                r#slot_1_occupied: false,
                r#slot_5_occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2262 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::South,
                r#slot_1_occupied: false,
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2170 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_4_occupied: false,
                r#facing: Facing::North,
                r#slot_1_occupied: false,
                r#slot_3_occupied: true,
                r#slot_5_occupied: false,
                r#slot_2_occupied: false,
            });
        }
        if state_id == 2312 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#facing: Facing::West,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_3_occupied: true,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2340 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
                r#facing: Facing::East,
                r#slot_0_occupied: true,
                r#slot_1_occupied: true,
                r#slot_5_occupied: false,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2237 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_5_occupied: true,
                r#facing: Facing::South,
                r#slot_1_occupied: false,
                r#slot_4_occupied: false,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2385 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#slot_3_occupied: true,
                r#facing: Facing::East,
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2303 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_3_occupied: true,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#facing: Facing::West,
                r#slot_2_occupied: true,
                r#slot_0_occupied: false,
            });
        }
        if state_id == 2347 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_4_occupied: true,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2232 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#facing: Facing::South,
                r#slot_0_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_3_occupied: true,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2241 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_3_occupied: true,
                r#slot_5_occupied: true,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#slot_4_occupied: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 2210 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: true,
                r#slot_0_occupied: true,
                r#facing: Facing::South,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_3_occupied: true,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2270 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::South,
                r#slot_3_occupied: false,
                r#slot_1_occupied: false,
                r#slot_4_occupied: false,
                r#slot_2_occupied: false,
                r#slot_0_occupied: false,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2302 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::West,
                r#slot_1_occupied: false,
                r#slot_4_occupied: false,
                r#slot_3_occupied: false,
                r#slot_5_occupied: false,
                r#slot_2_occupied: false,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2195 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_0_occupied: false,
                r#slot_1_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
                r#slot_2_occupied: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2185 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::North,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#slot_2_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2255 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
                r#facing: Facing::South,
                r#slot_4_occupied: true,
                r#slot_3_occupied: true,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2265 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_1_occupied: false,
                r#facing: Facing::South,
                r#slot_5_occupied: true,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2353 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_2_occupied: true,
                r#slot_3_occupied: true,
                r#slot_0_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2235 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_1_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
                r#slot_2_occupied: false,
                r#slot_5_occupied: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 2261 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#facing: Facing::South,
                r#slot_2_occupied: true,
                r#slot_0_occupied: false,
                r#slot_1_occupied: false,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2212 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#facing: Facing::South,
                r#slot_0_occupied: true,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
                r#slot_2_occupied: true,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2193 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::North,
                r#slot_1_occupied: false,
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
                r#slot_2_occupied: true,
                r#slot_5_occupied: true,
                r#slot_0_occupied: false,
            });
        }
        if state_id == 2360 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_3_occupied: true,
                r#slot_4_occupied: true,
                r#slot_0_occupied: true,
                r#slot_2_occupied: false,
                r#slot_5_occupied: false,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2395 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
                r#facing: Facing::East,
                r#slot_1_occupied: false,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2149 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::North,
                r#slot_0_occupied: true,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_2_occupied: true,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2179 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_2_occupied: true,
                r#slot_1_occupied: true,
                r#facing: Facing::North,
                r#slot_0_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2371 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
                r#facing: Facing::East,
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2201 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_1_occupied: false,
                r#slot_4_occupied: false,
                r#facing: Facing::North,
                r#slot_0_occupied: false,
                r#slot_5_occupied: true,
                r#slot_2_occupied: false,
            });
        }
        if state_id == 2379 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_0_occupied: false,
                r#slot_1_occupied: true,
                r#slot_4_occupied: true,
                r#slot_3_occupied: false,
                r#slot_2_occupied: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2380 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_5_occupied: false,
                r#slot_4_occupied: true,
                r#slot_1_occupied: true,
                r#slot_0_occupied: false,
            });
        }
        if state_id == 2253 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::South,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2397 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#slot_5_occupied: true,
                r#facing: Facing::East,
                r#slot_2_occupied: false,
                r#slot_0_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2233 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_0_occupied: true,
                r#facing: Facing::South,
                r#slot_2_occupied: false,
                r#slot_4_occupied: false,
                r#slot_1_occupied: false,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2311 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_3_occupied: true,
                r#facing: Facing::West,
                r#slot_4_occupied: true,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2144 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_1_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_0_occupied: true,
                r#slot_3_occupied: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2161 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_2_occupied: true,
                r#slot_3_occupied: true,
                r#slot_1_occupied: false,
                r#facing: Facing::North,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2236 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_3_occupied: false,
                r#slot_2_occupied: false,
                r#slot_0_occupied: true,
                r#slot_1_occupied: false,
                r#facing: Facing::South,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2165 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::North,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
                r#slot_0_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2359 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_5_occupied: true,
                r#slot_2_occupied: false,
                r#facing: Facing::East,
                r#slot_0_occupied: true,
                r#slot_1_occupied: false,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2249 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: true,
                r#slot_4_occupied: false,
                r#slot_0_occupied: false,
                r#slot_3_occupied: true,
                r#facing: Facing::South,
                r#slot_2_occupied: false,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2331 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_0_occupied: false,
                r#slot_5_occupied: true,
                r#slot_4_occupied: true,
                r#slot_1_occupied: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2301 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_1_occupied: false,
                r#facing: Facing::West,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_2_occupied: false,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2290 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_5_occupied: false,
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
                r#facing: Facing::West,
                r#slot_2_occupied: true,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2342 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_3_occupied: false,
                r#slot_1_occupied: true,
                r#slot_2_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 2378 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 2218 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#slot_1_occupied: true,
                r#facing: Facing::South,
                r#slot_0_occupied: true,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2296 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#facing: Facing::West,
                r#slot_0_occupied: true,
                r#slot_2_occupied: false,
                r#slot_1_occupied: false,
                r#slot_3_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2176 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_1_occupied: true,
                r#slot_4_occupied: true,
                r#facing: Facing::North,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2308 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#facing: Facing::West,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
                r#slot_5_occupied: false,
                r#slot_2_occupied: true,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2310 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::West,
                r#slot_0_occupied: false,
                r#slot_4_occupied: false,
                r#slot_3_occupied: false,
                r#slot_5_occupied: false,
                r#slot_2_occupied: true,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2321 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
                r#slot_1_occupied: false,
                r#slot_5_occupied: true,
                r#facing: Facing::West,
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2145 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::North,
                r#slot_0_occupied: true,
                r#slot_1_occupied: true,
                r#slot_2_occupied: true,
                r#slot_4_occupied: false,
                r#slot_3_occupied: true,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2207 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_3_occupied: true,
                r#slot_1_occupied: true,
                r#facing: Facing::South,
                r#slot_0_occupied: true,
                r#slot_5_occupied: true,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2376 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#facing: Facing::East,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
                r#slot_0_occupied: false,
                r#slot_4_occupied: true,
                r#slot_1_occupied: true,
            });
        }
        if state_id == 2363 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_0_occupied: true,
                r#slot_2_occupied: false,
                r#slot_1_occupied: false,
                r#slot_5_occupied: true,
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2208 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_1_occupied: true,
                r#facing: Facing::South,
                r#slot_0_occupied: true,
                r#slot_3_occupied: true,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2173 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_0_occupied: true,
                r#facing: Facing::North,
                r#slot_2_occupied: false,
                r#slot_1_occupied: false,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2199 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
                r#slot_1_occupied: false,
                r#slot_0_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 2257 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_3_occupied: true,
                r#slot_0_occupied: false,
                r#facing: Facing::South,
                r#slot_1_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2180 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: true,
                r#slot_2_occupied: true,
                r#slot_0_occupied: false,
                r#slot_4_occupied: true,
                r#slot_3_occupied: false,
                r#slot_5_occupied: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 2334 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: false,
                r#slot_0_occupied: false,
                r#slot_5_occupied: false,
                r#slot_1_occupied: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 2272 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#facing: Facing::West,
                r#slot_0_occupied: true,
                r#slot_1_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2369 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: true,
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#facing: Facing::East,
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2184 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#facing: Facing::North,
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
                r#slot_2_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2228 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_1_occupied: false,
                r#facing: Facing::South,
                r#slot_3_occupied: false,
                r#slot_2_occupied: true,
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2374 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_0_occupied: false,
                r#slot_4_occupied: false,
                r#slot_1_occupied: true,
                r#slot_3_occupied: false,
                r#slot_5_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2169 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::North,
                r#slot_1_occupied: false,
                r#slot_2_occupied: false,
                r#slot_0_occupied: true,
                r#slot_5_occupied: true,
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2150 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: true,
                r#facing: Facing::North,
                r#slot_2_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
                r#slot_0_occupied: true,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2206 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#facing: Facing::North,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_0_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2143 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_3_occupied: true,
                r#slot_5_occupied: true,
                r#facing: Facing::North,
                r#slot_1_occupied: true,
                r#slot_2_occupied: true,
                r#slot_0_occupied: true,
            });
        }
        if state_id == 2230 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_0_occupied: true,
                r#slot_3_occupied: false,
                r#facing: Facing::South,
                r#slot_5_occupied: false,
                r#slot_1_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2256 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
                r#facing: Facing::South,
                r#slot_3_occupied: true,
                r#slot_1_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2377 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_0_occupied: false,
                r#slot_3_occupied: true,
                r#facing: Facing::East,
                r#slot_2_occupied: false,
                r#slot_1_occupied: true,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2197 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_1_occupied: false,
                r#facing: Facing::North,
                r#slot_2_occupied: true,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2240 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_0_occupied: false,
                r#slot_2_occupied: true,
                r#slot_5_occupied: false,
                r#facing: Facing::South,
                r#slot_1_occupied: true,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2203 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: false,
                r#facing: Facing::North,
                r#slot_2_occupied: false,
                r#slot_0_occupied: false,
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2247 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::South,
                r#slot_1_occupied: true,
                r#slot_2_occupied: false,
                r#slot_0_occupied: false,
                r#slot_5_occupied: true,
                r#slot_4_occupied: true,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2370 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: true,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
                r#slot_3_occupied: true,
                r#slot_2_occupied: true,
                r#facing: Facing::East,
                r#slot_4_occupied: false,
            });
        }
        if state_id == 2361 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::East,
                r#slot_0_occupied: true,
                r#slot_1_occupied: false,
                r#slot_3_occupied: true,
                r#slot_4_occupied: false,
                r#slot_2_occupied: false,
                r#slot_5_occupied: true,
            });
        }
        if state_id == 2317 {
            return Some(ChiseledBookshelf {
                r#slot_1_occupied: true,
                r#slot_2_occupied: false,
                r#slot_0_occupied: false,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#facing: Facing::West,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2200 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: true,
                r#slot_1_occupied: false,
                r#facing: Facing::North,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
            });
        }
        if state_id == 2357 {
            return Some(ChiseledBookshelf {
                r#slot_0_occupied: true,
                r#slot_2_occupied: true,
                r#facing: Facing::East,
                r#slot_3_occupied: false,
                r#slot_5_occupied: true,
                r#slot_4_occupied: false,
                r#slot_1_occupied: false,
            });
        }
        if state_id == 2298 {
            return Some(ChiseledBookshelf {
                r#slot_4_occupied: false,
                r#slot_1_occupied: false,
                r#facing: Facing::West,
                r#slot_0_occupied: true,
                r#slot_2_occupied: false,
                r#slot_3_occupied: true,
                r#slot_5_occupied: false,
            });
        }
        if state_id == 2316 {
            return Some(ChiseledBookshelf {
                r#facing: Facing::West,
                r#slot_1_occupied: true,
                r#slot_2_occupied: false,
                r#slot_3_occupied: false,
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
                r#slot_4_occupied: true,
            });
        }
        if state_id == 2172 {
            return Some(ChiseledBookshelf {
                r#slot_3_occupied: false,
                r#slot_4_occupied: true,
                r#slot_1_occupied: false,
                r#slot_0_occupied: true,
                r#slot_5_occupied: false,
                r#facing: Facing::North,
                r#slot_2_occupied: false,
            });
        }
        if state_id == 2273 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_4_occupied: false,
                r#slot_5_occupied: true,
                r#slot_0_occupied: true,
                r#slot_3_occupied: true,
                r#slot_1_occupied: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 2260 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: false,
                r#slot_0_occupied: false,
                r#slot_1_occupied: false,
                r#slot_4_occupied: true,
                r#facing: Facing::South,
                r#slot_3_occupied: false,
                r#slot_2_occupied: true,
            });
        }
        if state_id == 2259 {
            return Some(ChiseledBookshelf {
                r#slot_2_occupied: true,
                r#slot_1_occupied: false,
                r#facing: Facing::South,
                r#slot_4_occupied: true,
                r#slot_5_occupied: true,
                r#slot_0_occupied: false,
                r#slot_3_occupied: false,
            });
        }
        if state_id == 2147 {
            return Some(ChiseledBookshelf {
                r#slot_5_occupied: true,
                r#slot_1_occupied: true,
                r#facing: Facing::North,
                r#slot_4_occupied: true,
                r#slot_2_occupied: true,
                r#slot_0_occupied: true,
                r#slot_3_occupied: false,
            });
        }
        return None;
    }
}

