use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fire {
    pub west: bool,
    pub north: bool,
    pub age: i32,
    pub east: bool,
    pub south: bool,
    pub up: bool,
}


impl BlockState for Fire {
    fn to_id(self) -> i32 {
        if block_state.r#south == false && block_state.r#age == 1 && block_state.r#up == true && block_state.r#west == false && block_state.r#east == false && block_state.r#north == false { return 3235; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#up == true && block_state.r#age == 9 && block_state.r#south == true && block_state.r#west == true { return 3470; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#north == true && block_state.r#age == 2 && block_state.r#up == true && block_state.r#west == true { return 3258; }
        if block_state.r#west == false && block_state.r#east == true && block_state.r#north == false && block_state.r#age == 1 && block_state.r#south == false && block_state.r#up == false { return 3221; }
        if block_state.r#north == false && block_state.r#up == true && block_state.r#east == true && block_state.r#south == true && block_state.r#age == 10 && block_state.r#west == true { return 3502; }
        if block_state.r#north == false && block_state.r#age == 14 && block_state.r#east == true && block_state.r#up == true && block_state.r#west == true && block_state.r#south == true { return 3630; }
        if block_state.r#age == 14 && block_state.r#north == true && block_state.r#east == false && block_state.r#south == true && block_state.r#up == false && block_state.r#west == false { return 3641; }
        if block_state.r#up == false && block_state.r#south == false && block_state.r#east == false && block_state.r#age == 3 && block_state.r#west == true && block_state.r#north == false { return 3300; }
        if block_state.r#up == false && block_state.r#north == true && block_state.r#age == 6 && block_state.r#west == true && block_state.r#south == false && block_state.r#east == false { return 3388; }
        if block_state.r#west == true && block_state.r#up == true && block_state.r#age == 14 && block_state.r#south == true && block_state.r#east == false && block_state.r#north == false { return 3646; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#age == 15 && block_state.r#north == false && block_state.r#up == false && block_state.r#west == true { return 3664; }
        if block_state.r#age == 2 && block_state.r#up == true && block_state.r#north == true && block_state.r#east == false && block_state.r#south == true && block_state.r#west == false { return 3255; }
        if block_state.r#age == 5 && block_state.r#up == true && block_state.r#east == false && block_state.r#south == false && block_state.r#north == true && block_state.r#west == true { return 3354; }
        if block_state.r#west == false && block_state.r#up == false && block_state.r#north == false && block_state.r#south == true && block_state.r#age == 1 && block_state.r#east == true { return 3217; }
        if block_state.r#age == 1 && block_state.r#up == true && block_state.r#east == true && block_state.r#north == false && block_state.r#south == true && block_state.r#west == true { return 3214; }
        if block_state.r#up == false && block_state.r#north == true && block_state.r#east == false && block_state.r#south == true && block_state.r#west == false && block_state.r#age == 6 { return 3385; }
        if block_state.r#east == true && block_state.r#up == false && block_state.r#west == false && block_state.r#age == 0 && block_state.r#south == true && block_state.r#north == false { return 3185; }
        if block_state.r#north == false && block_state.r#age == 15 && block_state.r#up == false && block_state.r#west == true && block_state.r#east == false && block_state.r#south == false { return 3684; }
        if block_state.r#east == true && block_state.r#west == false && block_state.r#up == false && block_state.r#north == true && block_state.r#south == true && block_state.r#age == 5 { return 3337; }
        if block_state.r#east == false && block_state.r#up == false && block_state.r#west == true && block_state.r#south == false && block_state.r#north == true && block_state.r#age == 9 { return 3484; }
        if block_state.r#east == false && block_state.r#age == 14 && block_state.r#south == true && block_state.r#up == true && block_state.r#west == true && block_state.r#north == true { return 3638; }
        if block_state.r#age == 7 && block_state.r#north == true && block_state.r#west == false && block_state.r#east == false && block_state.r#south == true && block_state.r#up == true { return 3415; }
        if block_state.r#west == true && block_state.r#south == false && block_state.r#up == true && block_state.r#age == 4 && block_state.r#east == true && block_state.r#north == false { return 3314; }
        if block_state.r#north == false && block_state.r#up == false && block_state.r#east == true && block_state.r#age == 6 && block_state.r#south == true && block_state.r#west == true { return 3376; }
        if block_state.r#age == 3 && block_state.r#north == true && block_state.r#east == true && block_state.r#south == false && block_state.r#up == true && block_state.r#west == false { return 3275; }
        if block_state.r#south == true && block_state.r#age == 3 && block_state.r#east == false && block_state.r#north == true && block_state.r#up == false && block_state.r#west == false { return 3289; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#west == false && block_state.r#north == true && block_state.r#up == true && block_state.r#age == 9 { return 3483; }
        if block_state.r#age == 12 && block_state.r#south == true && block_state.r#up == false && block_state.r#east == true && block_state.r#west == false && block_state.r#north == true { return 3561; }
        if block_state.r#north == false && block_state.r#west == false && block_state.r#age == 4 && block_state.r#east == true && block_state.r#south == false && block_state.r#up == true { return 3315; }
        if block_state.r#age == 13 && block_state.r#east == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#north == true { return 3591; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#west == true && block_state.r#south == false && block_state.r#age == 1 && block_state.r#up == true { return 3218; }
        if block_state.r#up == true && block_state.r#south == true && block_state.r#east == true && block_state.r#west == true && block_state.r#north == false && block_state.r#age == 5 { return 3342; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#south == true && block_state.r#east == true && block_state.r#up == false && block_state.r#age == 1 { return 3216; }
        if block_state.r#age == 11 && block_state.r#up == true && block_state.r#west == true && block_state.r#east == true && block_state.r#south == true && block_state.r#north == false { return 3534; }
        if block_state.r#south == true && block_state.r#age == 11 && block_state.r#up == true && block_state.r#west == false && block_state.r#north == false && block_state.r#east == true { return 3535; }
        if block_state.r#east == false && block_state.r#up == false && block_state.r#south == false && block_state.r#age == 6 && block_state.r#west == false && block_state.r#north == true { return 3389; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#age == 9 && block_state.r#east == true && block_state.r#up == true && block_state.r#west == false { return 3467; }
        if block_state.r#up == true && block_state.r#west == true && block_state.r#south == true && block_state.r#east == false && block_state.r#age == 12 && block_state.r#north == true { return 3574; }
        if block_state.r#age == 11 && block_state.r#south == false && block_state.r#up == false && block_state.r#east == false && block_state.r#west == false && block_state.r#north == false { return 3557; }
        if block_state.r#south == true && block_state.r#up == false && block_state.r#north == false && block_state.r#west == true && block_state.r#east == true && block_state.r#age == 13 { return 3600; }
        if block_state.r#age == 10 && block_state.r#south == false && block_state.r#east == false && block_state.r#west == true && block_state.r#north == true && block_state.r#up == false { return 3516; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#age == 11 && block_state.r#east == true && block_state.r#south == false && block_state.r#up == true { return 3538; }
        if block_state.r#west == true && block_state.r#age == 11 && block_state.r#south == false && block_state.r#east == false && block_state.r#north == false && block_state.r#up == true { return 3554; }
        if block_state.r#age == 4 && block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#west == true && block_state.r#up == true { return 3306; }
        if block_state.r#east == true && block_state.r#up == true && block_state.r#age == 12 && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false { return 3571; }
        if block_state.r#age == 13 && block_state.r#east == false && block_state.r#south == false && block_state.r#up == true && block_state.r#west == true && block_state.r#north == false { return 3618; }
        if block_state.r#age == 6 && block_state.r#east == true && block_state.r#up == true && block_state.r#south == false && block_state.r#north == true && block_state.r#west == true { return 3370; }
        if block_state.r#age == 14 && block_state.r#north == false && block_state.r#up == true && block_state.r#west == true && block_state.r#east == false && block_state.r#south == false { return 3650; }
        if block_state.r#age == 10 && block_state.r#west == true && block_state.r#up == false && block_state.r#north == true && block_state.r#south == false && block_state.r#east == true { return 3500; }
        if block_state.r#up == true && block_state.r#west == false && block_state.r#north == false && block_state.r#south == false && block_state.r#east == true && block_state.r#age == 5 { return 3347; }
        if block_state.r#up == false && block_state.r#east == true && block_state.r#north == true && block_state.r#west == false && block_state.r#south == false && block_state.r#age == 4 { return 3309; }
        if block_state.r#up == false && block_state.r#east == false && block_state.r#age == 11 && block_state.r#south == false && block_state.r#west == false && block_state.r#north == true { return 3549; }
        if block_state.r#up == true && block_state.r#north == false && block_state.r#age == 11 && block_state.r#west == false && block_state.r#south == false && block_state.r#east == false { return 3555; }
        if block_state.r#west == false && block_state.r#age == 14 && block_state.r#east == false && block_state.r#south == false && block_state.r#up == false && block_state.r#north == false { return 3653; }
        if block_state.r#south == true && block_state.r#age == 5 && block_state.r#north == false && block_state.r#west == true && block_state.r#east == true && block_state.r#up == false { return 3344; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == true && block_state.r#north == true && block_state.r#age == 5 { return 3334; }
        if block_state.r#age == 0 && block_state.r#west == false && block_state.r#east == true && block_state.r#south == false && block_state.r#north == true && block_state.r#up == true { return 3179; }
        if block_state.r#south == true && block_state.r#up == false && block_state.r#age == 7 && block_state.r#north == false && block_state.r#west == false && block_state.r#east == false { return 3425; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#age == 15 && block_state.r#west == true && block_state.r#east == true && block_state.r#up == false { return 3660; }
        if block_state.r#age == 7 && block_state.r#up == false && block_state.r#north == true && block_state.r#east == true && block_state.r#south == false && block_state.r#west == true { return 3404; }
        if block_state.r#age == 1 && block_state.r#north == true && block_state.r#west == false && block_state.r#south == true && block_state.r#east == false && block_state.r#up == false { return 3225; }
        if block_state.r#age == 1 && block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#up == true && block_state.r#west == true { return 3230; }
        if block_state.r#age == 1 && block_state.r#north == false && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false && block_state.r#up == false { return 3237; }
        if block_state.r#up == true && block_state.r#south == false && block_state.r#west == false && block_state.r#east == false && block_state.r#age == 15 && block_state.r#north == true { return 3675; }
        if block_state.r#up == true && block_state.r#age == 5 && block_state.r#east == false && block_state.r#west == false && block_state.r#north == false && block_state.r#south == false { return 3363; }
        if block_state.r#north == true && block_state.r#up == true && block_state.r#south == true && block_state.r#age == 5 && block_state.r#east == true && block_state.r#west == false { return 3335; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#up == true && block_state.r#west == true && block_state.r#age == 3 { return 3274; }
        if block_state.r#west == false && block_state.r#age == 11 && block_state.r#north == false && block_state.r#east == false && block_state.r#south == true && block_state.r#up == false { return 3553; }
        if block_state.r#south == false && block_state.r#age == 8 && block_state.r#north == false && block_state.r#up == false && block_state.r#west == false && block_state.r#east == true { return 3445; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#east == true && block_state.r#up == false && block_state.r#age == 0 && block_state.r#west == true { return 3180; }
        if block_state.r#up == true && block_state.r#west == true && block_state.r#age == 3 && block_state.r#east == false && block_state.r#north == true && block_state.r#south == false { return 3290; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#age == 5 && block_state.r#east == false && block_state.r#north == false && block_state.r#up == true { return 3362; }
        if block_state.r#age == 8 && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#up == true && block_state.r#west == false { return 3443; }
        if block_state.r#west == false && block_state.r#east == true && block_state.r#south == true && block_state.r#north == true && block_state.r#up == true && block_state.r#age == 15 { return 3655; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#south == false && block_state.r#age == 0 && block_state.r#up == false && block_state.r#west == false { return 3197; }
        if block_state.r#age == 0 && block_state.r#south == false && block_state.r#west == true && block_state.r#up == true && block_state.r#north == false && block_state.r#east == false { return 3202; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#south == false && block_state.r#age == 1 && block_state.r#up == true && block_state.r#north == true { return 3210; }
        if block_state.r#south == false && block_state.r#north == false && block_state.r#west == true && block_state.r#age == 6 && block_state.r#east == false && block_state.r#up == false { return 3396; }
        if block_state.r#north == true && block_state.r#up == false && block_state.r#west == false && block_state.r#age == 2 && block_state.r#east == true && block_state.r#south == true { return 3241; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#age == 13 && block_state.r#north == true && block_state.r#up == true && block_state.r#west == false { return 3595; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#west == false && block_state.r#age == 9 && block_state.r#north == false && block_state.r#up == false { return 3493; }
        if block_state.r#north == false && block_state.r#up == false && block_state.r#age == 4 && block_state.r#east == true && block_state.r#south == false && block_state.r#west == false { return 3317; }
        if block_state.r#south == false && block_state.r#up == true && block_state.r#north == true && block_state.r#east == false && block_state.r#west == true && block_state.r#age == 13 { return 3610; }
        if block_state.r#south == true && block_state.r#up == true && block_state.r#north == false && block_state.r#west == false && block_state.r#age == 7 && block_state.r#east == true { return 3407; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#up == false && block_state.r#age == 2 && block_state.r#west == false && block_state.r#north == true { return 3261; }
        if block_state.r#age == 10 && block_state.r#east == false && block_state.r#up == false && block_state.r#north == false && block_state.r#west == true && block_state.r#south == true { return 3520; }
        if block_state.r#west == false && block_state.r#up == false && block_state.r#age == 10 && block_state.r#north == true && block_state.r#south == false && block_state.r#east == false { return 3517; }
        if block_state.r#age == 12 && block_state.r#north == false && block_state.r#south == true && block_state.r#east == true && block_state.r#up == false && block_state.r#west == true { return 3568; }
        if block_state.r#north == true && block_state.r#age == 1 && block_state.r#west == true && block_state.r#east == true && block_state.r#up == true && block_state.r#south == true { return 3206; }
        if block_state.r#age == 6 && block_state.r#north == false && block_state.r#west == true && block_state.r#east == false && block_state.r#south == false && block_state.r#up == true { return 3394; }
        if block_state.r#age == 7 && block_state.r#south == true && block_state.r#east == false && block_state.r#up == false && block_state.r#west == true && block_state.r#north == false { return 3424; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#age == 14 && block_state.r#south == true && block_state.r#up == true && block_state.r#west == true { return 3622; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#age == 1 && block_state.r#south == true && block_state.r#up == false && block_state.r#west == true { return 3232; }
        if block_state.r#age == 9 && block_state.r#south == false && block_state.r#up == true && block_state.r#north == false && block_state.r#west == false && block_state.r#east == true { return 3475; }
        if block_state.r#age == 1 && block_state.r#north == true && block_state.r#east == false && block_state.r#west == true && block_state.r#south == true && block_state.r#up == true { return 3222; }
        if block_state.r#west == true && block_state.r#up == true && block_state.r#south == false && block_state.r#east == true && block_state.r#age == 2 && block_state.r#north == true { return 3242; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#east == true && block_state.r#age == 7 && block_state.r#up == true && block_state.r#west == false { return 3411; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#south == true && block_state.r#east == false && block_state.r#up == false && block_state.r#age == 9 { return 3489; }
        if block_state.r#west == true && block_state.r#up == true && block_state.r#age == 10 && block_state.r#north == false && block_state.r#east == false && block_state.r#south == true { return 3518; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#age == 0 && block_state.r#north == false && block_state.r#up == false && block_state.r#west == true { return 3184; }
        if block_state.r#west == false && block_state.r#age == 14 && block_state.r#east == true && block_state.r#south == true && block_state.r#north == false && block_state.r#up == false { return 3633; }
        if block_state.r#age == 3 && block_state.r#north == false && block_state.r#west == false && block_state.r#south == true && block_state.r#up == false && block_state.r#east == true { return 3281; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#up == false && block_state.r#west == true && block_state.r#north == false && block_state.r#age == 4 { return 3316; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#west == false && block_state.r#up == true && block_state.r#east == false && block_state.r#age == 13 { return 3607; }
        if block_state.r#age == 10 && block_state.r#north == true && block_state.r#east == false && block_state.r#south == false && block_state.r#up == true && block_state.r#west == true { return 3514; }
        if block_state.r#north == false && block_state.r#age == 6 && block_state.r#east == false && block_state.r#up == false && block_state.r#south == true && block_state.r#west == false { return 3393; }
        if block_state.r#up == false && block_state.r#age == 15 && block_state.r#north == true && block_state.r#south == true && block_state.r#west == false && block_state.r#east == false { return 3673; }
        if block_state.r#south == false && block_state.r#up == true && block_state.r#west == true && block_state.r#east == false && block_state.r#age == 7 && block_state.r#north == true { return 3418; }
        if block_state.r#age == 3 && block_state.r#north == true && block_state.r#west == false && block_state.r#south == false && block_state.r#east == false && block_state.r#up == true { return 3291; }
        if block_state.r#age == 13 && block_state.r#south == false && block_state.r#north == true && block_state.r#up == true && block_state.r#east == false && block_state.r#west == false { return 3611; }
        if block_state.r#east == false && block_state.r#age == 6 && block_state.r#up == true && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false { return 3395; }
        if block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#north == true && block_state.r#east == false && block_state.r#age == 9 { return 3479; }
        if block_state.r#west == true && block_state.r#age == 1 && block_state.r#up == false && block_state.r#north == false && block_state.r#south == false && block_state.r#east == false { return 3236; }
        if block_state.r#east == false && block_state.r#up == false && block_state.r#age == 12 && block_state.r#west == false && block_state.r#north == true && block_state.r#south == true { return 3577; }
        if block_state.r#east == false && block_state.r#up == false && block_state.r#west == true && block_state.r#north == false && block_state.r#age == 5 && block_state.r#south == false { return 3364; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#south == true && block_state.r#up == true && block_state.r#east == true && block_state.r#age == 11 { return 3527; }
        if block_state.r#age == 11 && block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#west == false && block_state.r#up == true { return 3531; }
        if block_state.r#west == false && block_state.r#up == false && block_state.r#east == false && block_state.r#age == 7 && block_state.r#north == false && block_state.r#south == false { return 3429; }
        if block_state.r#east == true && block_state.r#age == 2 && block_state.r#north == true && block_state.r#south == false && block_state.r#up == true && block_state.r#west == false { return 3243; }
        if block_state.r#age == 14 && block_state.r#east == true && block_state.r#south == false && block_state.r#up == true && block_state.r#west == true && block_state.r#north == false { return 3634; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#age == 0 && block_state.r#up == true && block_state.r#west == true && block_state.r#north == false { return 3186; }
        if block_state.r#south == false && block_state.r#up == true && block_state.r#west == true && block_state.r#age == 9 && block_state.r#east == true && block_state.r#north == true { return 3466; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#age == 4 && block_state.r#north == false { return 3327; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#up == false && block_state.r#west == false && block_state.r#age == 0 && block_state.r#east == true { return 3181; }
        if block_state.r#south == false && block_state.r#age == 14 && block_state.r#up == false && block_state.r#east == true && block_state.r#north == false && block_state.r#west == false { return 3637; }
        if block_state.r#south == false && block_state.r#up == false && block_state.r#west == true && block_state.r#east == false && block_state.r#age == 0 && block_state.r#north == true { return 3196; }
        if block_state.r#west == true && block_state.r#age == 14 && block_state.r#north == true && block_state.r#east == true && block_state.r#south == false && block_state.r#up == false { return 3628; }
        if block_state.r#east == true && block_state.r#up == false && block_state.r#west == false && block_state.r#age == 8 && block_state.r#south == false && block_state.r#north == true { return 3437; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#north == false && block_state.r#west == true && block_state.r#up == false && block_state.r#age == 5 { return 3360; }
        if block_state.r#south == false && block_state.r#age == 9 && block_state.r#east == false && block_state.r#up == true && block_state.r#north == true && block_state.r#west == true { return 3482; }
        if block_state.r#west == false && block_state.r#east == true && block_state.r#south == true && block_state.r#age == 13 && block_state.r#north == true && block_state.r#up == false { return 3593; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#north == true && block_state.r#age == 13 && block_state.r#up == false && block_state.r#west == false { return 3613; }
        if block_state.r#north == false && block_state.r#up == true && block_state.r#west == true && block_state.r#age == 9 && block_state.r#east == false && block_state.r#south == false { return 3490; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#age == 3 && block_state.r#up == true && block_state.r#west == false && block_state.r#north == false { return 3279; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#up == false && block_state.r#west == true && block_state.r#north == false && block_state.r#age == 14 { return 3648; }
        if block_state.r#east == false && block_state.r#age == 8 && block_state.r#south == true && block_state.r#north == true && block_state.r#up == true && block_state.r#west == false { return 3447; }
        if block_state.r#north == false && block_state.r#age == 1 && block_state.r#east == true && block_state.r#south == false && block_state.r#up == false && block_state.r#west == true { return 3220; }
        if block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#east == true && block_state.r#north == false && block_state.r#age == 15 { return 3663; }
        if block_state.r#up == true && block_state.r#west == true && block_state.r#north == true && block_state.r#age == 15 && block_state.r#south == true && block_state.r#east == false { return 3670; }
        if block_state.r#age == 11 && block_state.r#north == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#east == false { return 3543; }
        if block_state.r#age == 13 && block_state.r#south == false && block_state.r#up == true && block_state.r#west == true && block_state.r#north == false && block_state.r#east == true { return 3602; }
        if block_state.r#up == true && block_state.r#west == true && block_state.r#south == false && block_state.r#east == true && block_state.r#north == false && block_state.r#age == 8 { return 3442; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#east == true && block_state.r#up == false && block_state.r#age == 2 && block_state.r#north == true { return 3244; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#east == true && block_state.r#age == 4 && block_state.r#north == true && block_state.r#up == true { return 3307; }
        if block_state.r#east == false && block_state.r#age == 4 && block_state.r#up == true && block_state.r#north == true && block_state.r#south == true && block_state.r#west == true { return 3318; }
        if block_state.r#west == false && block_state.r#up == false && block_state.r#south == true && block_state.r#north == true && block_state.r#age == 6 && block_state.r#east == true { return 3369; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#up == true && block_state.r#west == true && block_state.r#north == false && block_state.r#age == 7 { return 3426; }
        if block_state.r#west == true && block_state.r#east == false && block_state.r#south == true && block_state.r#north == true && block_state.r#age == 14 && block_state.r#up == false { return 3640; }
        if block_state.r#west == true && block_state.r#up == false && block_state.r#south == false && block_state.r#east == true && block_state.r#age == 3 && block_state.r#north == true { return 3276; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#up == false && block_state.r#age == 5 && block_state.r#north == false && block_state.r#east == true { return 3349; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#age == 11 && block_state.r#up == false && block_state.r#east == true && block_state.r#north == true { return 3528; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#east == true && block_state.r#age == 4 && block_state.r#up == true && block_state.r#south == true { return 3311; }
        if block_state.r#up == false && block_state.r#north == true && block_state.r#south == false && block_state.r#east == false && block_state.r#west == true && block_state.r#age == 11 { return 3548; }
        if block_state.r#age == 10 && block_state.r#east == false && block_state.r#north == true && block_state.r#south == true && block_state.r#west == false && block_state.r#up == false { return 3513; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#up == false && block_state.r#south == true && block_state.r#east == false && block_state.r#age == 2 { return 3256; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#age == 12 && block_state.r#south == true && block_state.r#up == true && block_state.r#east == true { return 3558; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#south == false && block_state.r#east == true && block_state.r#age == 6 && block_state.r#up == false { return 3372; }
        if block_state.r#south == false && block_state.r#age == 9 && block_state.r#north == false && block_state.r#up == true && block_state.r#west == false && block_state.r#east == false { return 3491; }
        if block_state.r#age == 4 && block_state.r#west == true && block_state.r#north == true && block_state.r#east == false && block_state.r#south == true && block_state.r#up == false { return 3320; }
        if block_state.r#age == 1 && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false && block_state.r#up == false && block_state.r#north == true { return 3229; }
        if block_state.r#age == 0 && block_state.r#north == true && block_state.r#south == true && block_state.r#east == false && block_state.r#up == true && block_state.r#west == true { return 3190; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#north == false && block_state.r#east == false && block_state.r#age == 8 && block_state.r#up == true { return 3458; }
        if block_state.r#west == false && block_state.r#up == true && block_state.r#east == true && block_state.r#age == 11 && block_state.r#north == false && block_state.r#south == false { return 3539; }
        if block_state.r#up == false && block_state.r#south == true && block_state.r#north == true && block_state.r#west == false && block_state.r#age == 5 && block_state.r#east == false { return 3353; }
        if block_state.r#up == true && block_state.r#west == false && block_state.r#age == 15 && block_state.r#north == false && block_state.r#east == false && block_state.r#south == false { return 3683; }
        if block_state.r#south == true && block_state.r#up == false && block_state.r#age == 4 && block_state.r#north == true && block_state.r#west == false && block_state.r#east == true { return 3305; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#up == false && block_state.r#east == false && block_state.r#north == true && block_state.r#age == 8 { return 3453; }
        if block_state.r#up == true && block_state.r#south == false && block_state.r#north == true && block_state.r#age == 13 && block_state.r#east == true && block_state.r#west == true { return 3594; }
        if block_state.r#east == true && block_state.r#age == 2 && block_state.r#north == false && block_state.r#south == true && block_state.r#west == false && block_state.r#up == false { return 3249; }
        if block_state.r#age == 0 && block_state.r#east == true && block_state.r#south == true && block_state.r#west == true && block_state.r#north == true && block_state.r#up == true { return 3174; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#age == 3 && block_state.r#east == true && block_state.r#up == false && block_state.r#south == false { return 3284; }
        if block_state.r#age == 4 && block_state.r#south == true && block_state.r#west == true && block_state.r#north == false && block_state.r#up == true && block_state.r#east == true { return 3310; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#south == true && block_state.r#east == false && block_state.r#up == true && block_state.r#age == 12 { return 3582; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#north == false && block_state.r#age == 0 && block_state.r#up == true && block_state.r#east == true { return 3182; }
        if block_state.r#up == true && block_state.r#north == false && block_state.r#age == 9 && block_state.r#south == true && block_state.r#west == false && block_state.r#east == false { return 3487; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#south == false && block_state.r#up == false && block_state.r#age == 4 && block_state.r#west == false { return 3325; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#north == true && block_state.r#up == false && block_state.r#age == 0 && block_state.r#south == true { return 3193; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#south == true && block_state.r#up == true && block_state.r#age == 5 && block_state.r#east == false { return 3359; }
        if block_state.r#north == false && block_state.r#age == 10 && block_state.r#up == true && block_state.r#west == false && block_state.r#east == true && block_state.r#south == true { return 3503; }
        if block_state.r#age == 1 && block_state.r#east == false && block_state.r#south == false && block_state.r#up == true && block_state.r#west == true && block_state.r#north == false { return 3234; }
        if block_state.r#up == true && block_state.r#north == false && block_state.r#west == true && block_state.r#age == 3 && block_state.r#south == false && block_state.r#east == false { return 3298; }
        if block_state.r#west == true && block_state.r#up == false && block_state.r#age == 11 && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false { return 3540; }
        if block_state.r#south == true && block_state.r#age == 13 && block_state.r#west == false && block_state.r#north == false && block_state.r#east == false && block_state.r#up == false { return 3617; }
        if block_state.r#up == false && block_state.r#age == 10 && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#west == true { return 3508; }
        if block_state.r#age == 2 && block_state.r#up == true && block_state.r#east == false && block_state.r#west == false && block_state.r#north == true && block_state.r#south == false { return 3259; }
        if block_state.r#age == 4 && block_state.r#up == true && block_state.r#west == false && block_state.r#east == false && block_state.r#north == true && block_state.r#south == false { return 3323; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#south == false && block_state.r#up == false && block_state.r#age == 4 && block_state.r#east == false { return 3332; }
        if block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#east == true && block_state.r#north == true && block_state.r#age == 0 { return 3175; }
        if block_state.r#age == 11 && block_state.r#east == false && block_state.r#west == true && block_state.r#south == true && block_state.r#up == false && block_state.r#north == false { return 3552; }
        if block_state.r#up == false && block_state.r#age == 3 && block_state.r#west == false && block_state.r#north == true && block_state.r#south == false && block_state.r#east == false { return 3293; }
        if block_state.r#north == true && block_state.r#up == false && block_state.r#age == 10 && block_state.r#south == true && block_state.r#east == true && block_state.r#west == false { return 3497; }
        if block_state.r#up == true && block_state.r#age == 5 && block_state.r#north == false && block_state.r#east == true && block_state.r#south == false && block_state.r#west == true { return 3346; }
        if block_state.r#south == false && block_state.r#north == false && block_state.r#east == true && block_state.r#age == 12 && block_state.r#up == false && block_state.r#west == false { return 3573; }
        if block_state.r#up == false && block_state.r#east == true && block_state.r#north == false && block_state.r#west == true && block_state.r#south == false && block_state.r#age == 13 { return 3604; }
        if block_state.r#age == 0 && block_state.r#east == false && block_state.r#north == false && block_state.r#up == false && block_state.r#south == true && block_state.r#west == true { return 3200; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#east == true && block_state.r#age == 8 && block_state.r#north == true && block_state.r#up == true { return 3430; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#age == 3 && block_state.r#up == true && block_state.r#south == false && block_state.r#west == false { return 3283; }
        if block_state.r#age == 13 && block_state.r#east == true && block_state.r#up == false && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true { return 3601; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#up == false && block_state.r#west == true && block_state.r#age == 4 && block_state.r#east == false { return 3328; }
        if block_state.r#up == false && block_state.r#east == true && block_state.r#age == 3 && block_state.r#south == false && block_state.r#west == false && block_state.r#north == false { return 3285; }
        if block_state.r#west == false && block_state.r#east == true && block_state.r#south == false && block_state.r#north == false && block_state.r#age == 10 && block_state.r#up == false { return 3509; }
        if block_state.r#age == 6 && block_state.r#up == true && block_state.r#north == true && block_state.r#west == false && block_state.r#south == false && block_state.r#east == true { return 3371; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#north == true && block_state.r#up == false && block_state.r#age == 14 && block_state.r#west == false { return 3629; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#up == false && block_state.r#west == true && block_state.r#north == false && block_state.r#age == 15 { return 3680; }
        if block_state.r#east == true && block_state.r#age == 6 && block_state.r#west == false && block_state.r#south == false && block_state.r#north == false && block_state.r#up == true { return 3379; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#age == 6 && block_state.r#east == false && block_state.r#up == true && block_state.r#south == true { return 3382; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#age == 14 && block_state.r#up == false && block_state.r#west == false && block_state.r#north == true { return 3645; }
        if block_state.r#up == true && block_state.r#east == false && block_state.r#south == true && block_state.r#age == 6 && block_state.r#west == false && block_state.r#north == true { return 3383; }
        if block_state.r#age == 5 && block_state.r#north == false && block_state.r#south == true && block_state.r#up == true && block_state.r#west == true && block_state.r#east == false { return 3358; }
        if block_state.r#south == true && block_state.r#age == 9 && block_state.r#north == false && block_state.r#east == true && block_state.r#up == false && block_state.r#west == false { return 3473; }
        if block_state.r#up == false && block_state.r#south == true && block_state.r#east == true && block_state.r#north == false && block_state.r#age == 7 && block_state.r#west == false { return 3409; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#up == true && block_state.r#east == false && block_state.r#age == 2 && block_state.r#west == false { return 3267; }
        if block_state.r#north == false && block_state.r#up == true && block_state.r#age == 15 && block_state.r#west == true && block_state.r#south == false && block_state.r#east == false { return 3682; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#age == 11 && block_state.r#south == true && block_state.r#north == true && block_state.r#up == true { return 3526; }
        if block_state.r#south == true && block_state.r#age == 0 && block_state.r#east == true && block_state.r#north == false && block_state.r#up == true && block_state.r#west == false { return 3183; }
        if block_state.r#age == 14 && block_state.r#north == false && block_state.r#west == false && block_state.r#east == true && block_state.r#south == false && block_state.r#up == true { return 3635; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#age == 15 && block_state.r#up == false && block_state.r#west == true && block_state.r#east == false { return 3672; }
        if block_state.r#south == false && block_state.r#up == false && block_state.r#west == false && block_state.r#north == true && block_state.r#age == 3 && block_state.r#east == true { return 3277; }
        if block_state.r#west == true && block_state.r#age == 11 && block_state.r#east == false && block_state.r#south == true && block_state.r#north == false && block_state.r#up == true { return 3550; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#age == 13 && block_state.r#east == true && block_state.r#north == false && block_state.r#up == true { return 3603; }
        if block_state.r#up == true && block_state.r#north == true && block_state.r#west == false && block_state.r#east == false && block_state.r#south == true && block_state.r#age == 15 { return 3671; }
        if block_state.r#up == true && block_state.r#east == true && block_state.r#age == 14 && block_state.r#south == false && block_state.r#north == true && block_state.r#west == true { return 3626; }
        if block_state.r#east == true && block_state.r#up == true && block_state.r#west == false && block_state.r#north == true && block_state.r#south == false && block_state.r#age == 1 { return 3211; }
        if block_state.r#up == false && block_state.r#west == true && block_state.r#age == 3 && block_state.r#east == false && block_state.r#north == true && block_state.r#south == false { return 3292; }
        if block_state.r#west == true && block_state.r#age == 13 && block_state.r#south == false && block_state.r#east == true && block_state.r#north == true && block_state.r#up == false { return 3596; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#north == false && block_state.r#west == false && block_state.r#up == false && block_state.r#age == 15 { return 3665; }
        if block_state.r#up == false && block_state.r#age == 8 && block_state.r#east == false && block_state.r#north == false && block_state.r#west == true && block_state.r#south == true { return 3456; }
        if block_state.r#up == false && block_state.r#east == false && block_state.r#south == true && block_state.r#west == true && block_state.r#age == 10 && block_state.r#north == true { return 3512; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#up == false && block_state.r#west == false && block_state.r#age == 12 { return 3585; }
        if block_state.r#south == false && block_state.r#up == false && block_state.r#east == false && block_state.r#west == false && block_state.r#age == 2 && block_state.r#north == false { return 3269; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#up == false && block_state.r#west == true && block_state.r#age == 12 && block_state.r#north == true { return 3560; }
        if block_state.r#age == 15 && block_state.r#up == true && block_state.r#north == false && block_state.r#east == true && block_state.r#south == false && block_state.r#west == false { return 3667; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#north == false && block_state.r#age == 15 && block_state.r#east == false && block_state.r#up == false { return 3681; }
        if block_state.r#south == false && block_state.r#up == false && block_state.r#age == 14 && block_state.r#west == true && block_state.r#north == false && block_state.r#east == true { return 3636; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#south == true && block_state.r#age == 15 && block_state.r#up == false && block_state.r#east == true { return 3657; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#south == false && block_state.r#age == 9 && block_state.r#up == false && block_state.r#west == true { return 3492; }
        if block_state.r#age == 0 && block_state.r#east == true && block_state.r#south == false && block_state.r#north == true && block_state.r#up == true && block_state.r#west == true { return 3178; }
        if block_state.r#south == true && block_state.r#up == false && block_state.r#west == true && block_state.r#age == 9 && block_state.r#east == true && block_state.r#north == true { return 3464; }
        if block_state.r#west == true && block_state.r#age == 6 && block_state.r#east == false && block_state.r#north == true && block_state.r#south == false && block_state.r#up == true { return 3386; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#up == true && block_state.r#west == true && block_state.r#age == 10 && block_state.r#south == false { return 3506; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#up == false && block_state.r#west == true && block_state.r#age == 7 && block_state.r#north == false { return 3412; }
        if block_state.r#east == true && block_state.r#up == true && block_state.r#north == false && block_state.r#west == false && block_state.r#south == true && block_state.r#age == 8 { return 3439; }
        if block_state.r#age == 8 && block_state.r#south == true && block_state.r#north == false && block_state.r#east == true && block_state.r#up == false && block_state.r#west == false { return 3441; }
        if block_state.r#south == true && block_state.r#up == false && block_state.r#west == true && block_state.r#east == false && block_state.r#age == 11 && block_state.r#north == true { return 3544; }
        if block_state.r#south == true && block_state.r#up == false && block_state.r#west == true && block_state.r#age == 0 && block_state.r#east == true && block_state.r#north == true { return 3176; }
        if block_state.r#west == false && block_state.r#age == 12 && block_state.r#north == true && block_state.r#up == true && block_state.r#south == true && block_state.r#east == false { return 3575; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#age == 9 && block_state.r#up == false && block_state.r#east == false && block_state.r#west == true { return 3480; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#age == 15 && block_state.r#north == true && block_state.r#east == true && block_state.r#up == false { return 3661; }
        if block_state.r#south == false && block_state.r#east == false && block_state.r#up == true && block_state.r#age == 6 && block_state.r#west == false && block_state.r#north == true { return 3387; }
        if block_state.r#age == 10 && block_state.r#south == true && block_state.r#up == true && block_state.r#north == true && block_state.r#east == false && block_state.r#west == false { return 3511; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#age == 14 && block_state.r#up == false && block_state.r#west == true && block_state.r#north == false { return 3652; }
        if block_state.r#up == true && block_state.r#age == 8 && block_state.r#south == true && block_state.r#west == true && block_state.r#east == true && block_state.r#north == false { return 3438; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#south == false && block_state.r#east == false && block_state.r#age == 14 && block_state.r#up == true { return 3642; }
        if block_state.r#age == 8 && block_state.r#east == false && block_state.r#up == false && block_state.r#west == false && block_state.r#south == true && block_state.r#north == true { return 3449; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#west == true && block_state.r#north == false && block_state.r#up == true && block_state.r#age == 9 { return 3474; }
        if block_state.r#west == false && block_state.r#south == false && block_state.r#up == false && block_state.r#age == 7 && block_state.r#east == true && block_state.r#north == false { return 3413; }
        if block_state.r#up == true && block_state.r#east == false && block_state.r#north == true && block_state.r#south == true && block_state.r#west == true && block_state.r#age == 3 { return 3286; }
        if block_state.r#west == true && block_state.r#east == false && block_state.r#age == 7 && block_state.r#south == false && block_state.r#north == false && block_state.r#up == false { return 3428; }
        if block_state.r#north == false && block_state.r#age == 3 && block_state.r#south == false && block_state.r#east == false && block_state.r#west == false && block_state.r#up == true { return 3299; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#up == true && block_state.r#west == false && block_state.r#age == 13 && block_state.r#north == false { return 3599; }
        if block_state.r#age == 14 && block_state.r#south == true && block_state.r#east == true && block_state.r#up == false && block_state.r#north == true && block_state.r#west == true { return 3624; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#north == true && block_state.r#age == 7 && block_state.r#south == false && block_state.r#up == false { return 3421; }
        if block_state.r#south == true && block_state.r#up == false && block_state.r#east == true && block_state.r#west == false && block_state.r#age == 5 && block_state.r#north == false { return 3345; }
        if block_state.r#up == true && block_state.r#east == false && block_state.r#south == true && block_state.r#age == 5 && block_state.r#north == true && block_state.r#west == true { return 3350; }
        if block_state.r#age == 15 && block_state.r#east == false && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#north == false { return 3679; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#west == true && block_state.r#up == false && block_state.r#age == 3 && block_state.r#north == true { return 3288; }
        if block_state.r#east == false && block_state.r#up == false && block_state.r#north == false && block_state.r#age == 15 && block_state.r#west == false && block_state.r#south == false { return 3685; }
        if block_state.r#up == true && block_state.r#north == true && block_state.r#east == true && block_state.r#age == 14 && block_state.r#south == false && block_state.r#west == false { return 3627; }
        if block_state.r#north == true && block_state.r#up == true && block_state.r#east == false && block_state.r#age == 4 && block_state.r#south == false && block_state.r#west == true { return 3322; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#west == true && block_state.r#south == false && block_state.r#up == true && block_state.r#age == 8 { return 3450; }
        if block_state.r#up == true && block_state.r#age == 13 && block_state.r#north == false && block_state.r#east == false && block_state.r#west == true && block_state.r#south == true { return 3614; }
        if block_state.r#south == true && block_state.r#age == 4 && block_state.r#up == false && block_state.r#west == true && block_state.r#east == true && block_state.r#north == true { return 3304; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#west == true && block_state.r#north == true && block_state.r#age == 10 && block_state.r#up == true { return 3498; }
        if block_state.r#age == 0 && block_state.r#west == false && block_state.r#south == false && block_state.r#north == false && block_state.r#up == true && block_state.r#east == false { return 3203; }
        if block_state.r#age == 2 && block_state.r#north == true && block_state.r#east == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false { return 3239; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#up == false && block_state.r#west == true && block_state.r#east == true && block_state.r#age == 2 { return 3248; }
        if block_state.r#west == false && block_state.r#up == false && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#age == 15 { return 3669; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#age == 7 && block_state.r#south == true && block_state.r#up == true && block_state.r#east == false { return 3422; }
        if block_state.r#age == 8 && block_state.r#east == false && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false && block_state.r#up == true { return 3459; }
        if block_state.r#south == true && block_state.r#age == 4 && block_state.r#west == false && block_state.r#east == true && block_state.r#north == true && block_state.r#up == true { return 3303; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#up == true && block_state.r#south == false && block_state.r#east == true && block_state.r#age == 2 { return 3250; }
        if block_state.r#north == true && block_state.r#age == 15 && block_state.r#south == false && block_state.r#east == false && block_state.r#up == true && block_state.r#west == true { return 3674; }
        if block_state.r#south == true && block_state.r#up == false && block_state.r#east == false && block_state.r#west == true && block_state.r#north == true && block_state.r#age == 0 { return 3192; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#east == true && block_state.r#up == false && block_state.r#west == true && block_state.r#age == 9 { return 3468; }
        if block_state.r#west == true && block_state.r#up == false && block_state.r#north == true && block_state.r#south == true && block_state.r#east == false && block_state.r#age == 13 { return 3608; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#up == false && block_state.r#age == 8 && block_state.r#east == false && block_state.r#west == false { return 3461; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#south == true && block_state.r#up == false && block_state.r#north == false && block_state.r#age == 11 { return 3536; }
        if block_state.r#east == false && block_state.r#up == true && block_state.r#south == true && block_state.r#west == false && block_state.r#age == 0 && block_state.r#north == false { return 3199; }
        if block_state.r#age == 0 && block_state.r#east == false && block_state.r#north == true && block_state.r#west == true && block_state.r#up == true && block_state.r#south == false { return 3194; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#up == true && block_state.r#west == false && block_state.r#age == 5 && block_state.r#east == false { return 3351; }
        if block_state.r#east == true && block_state.r#age == 6 && block_state.r#south == true && block_state.r#north == false && block_state.r#up == true && block_state.r#west == false { return 3375; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#age == 6 && block_state.r#west == false && block_state.r#up == false { return 3381; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#age == 3 { return 3295; }
        if block_state.r#age == 7 && block_state.r#up == true && block_state.r#east == true && block_state.r#north == false && block_state.r#west == true && block_state.r#south == false { return 3410; }
        if block_state.r#age == 8 && block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#up == true && block_state.r#west == false { return 3435; }
        if block_state.r#age == 0 && block_state.r#north == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#east == false { return 3191; }
        if block_state.r#up == true && block_state.r#east == false && block_state.r#north == false && block_state.r#west == false && block_state.r#age == 4 && block_state.r#south == false { return 3331; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#west == true && block_state.r#south == true && block_state.r#age == 10 && block_state.r#up == false { return 3496; }
        if block_state.r#north == false && block_state.r#west == true && block_state.r#east == true && block_state.r#south == true && block_state.r#up == false && block_state.r#age == 7 { return 3408; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#west == true && block_state.r#south == false && block_state.r#up == true && block_state.r#age == 11 { return 3530; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#north == true && block_state.r#up == false && block_state.r#age == 11 && block_state.r#east == true { return 3533; }
        if block_state.r#east == true && block_state.r#west == false && block_state.r#south == false && block_state.r#up == false && block_state.r#age == 5 && block_state.r#north == true { return 3341; }
        if block_state.r#west == false && block_state.r#up == true && block_state.r#age == 14 && block_state.r#east == false && block_state.r#north == false && block_state.r#south == true { return 3647; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#north == true && block_state.r#up == true && block_state.r#west == false && block_state.r#age == 7 { return 3399; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#west == false && block_state.r#east == false && block_state.r#up == false && block_state.r#age == 5 { return 3361; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == true && block_state.r#age == 15 { return 3654; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#age == 2 && block_state.r#south == true && block_state.r#up == false && block_state.r#west == true { return 3240; }
        if block_state.r#east == true && block_state.r#age == 2 && block_state.r#south == false && block_state.r#up == true && block_state.r#north == false && block_state.r#west == false { return 3251; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#up == false && block_state.r#north == true && block_state.r#age == 2 && block_state.r#east == false { return 3257; }
        if block_state.r#age == 5 && block_state.r#east == true && block_state.r#west == true && block_state.r#south == true && block_state.r#north == true && block_state.r#up == false { return 3336; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#up == false && block_state.r#west == true && block_state.r#south == false && block_state.r#age == 9 { return 3476; }
        if block_state.r#west == true && block_state.r#age == 10 && block_state.r#north == true && block_state.r#east == true && block_state.r#south == true && block_state.r#up == true { return 3494; }
        if block_state.r#up == true && block_state.r#north == true && block_state.r#west == true && block_state.r#east == true && block_state.r#age == 15 && block_state.r#south == false { return 3658; }
        if block_state.r#south == false && block_state.r#east == true && block_state.r#up == true && block_state.r#age == 12 && block_state.r#north == true && block_state.r#west == false { return 3563; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#east == false && block_state.r#north == true && block_state.r#age == 11 && block_state.r#up == true { return 3547; }
        if block_state.r#age == 12 && block_state.r#east == false && block_state.r#up == false && block_state.r#north == true && block_state.r#west == true && block_state.r#south == false { return 3580; }
        if block_state.r#east == false && block_state.r#up == false && block_state.r#age == 12 && block_state.r#north == false && block_state.r#south == false && block_state.r#west == false { return 3589; }
        if block_state.r#up == true && block_state.r#west == true && block_state.r#age == 2 && block_state.r#north == true && block_state.r#east == true && block_state.r#south == true { return 3238; }
        if block_state.r#north == false && block_state.r#up == true && block_state.r#south == true && block_state.r#age == 6 && block_state.r#east == true && block_state.r#west == true { return 3374; }
        if block_state.r#age == 12 && block_state.r#up == true && block_state.r#north == false && block_state.r#west == true && block_state.r#east == true && block_state.r#south == true { return 3566; }
        if block_state.r#east == false && block_state.r#up == false && block_state.r#age == 13 && block_state.r#north == false && block_state.r#south == true && block_state.r#west == true { return 3616; }
        if block_state.r#age == 9 && block_state.r#east == false && block_state.r#north == true && block_state.r#west == true && block_state.r#south == true && block_state.r#up == true { return 3478; }
        if block_state.r#age == 2 && block_state.r#up == true && block_state.r#east == true && block_state.r#north == false && block_state.r#west == false && block_state.r#south == true { return 3247; }
        if block_state.r#up == false && block_state.r#east == false && block_state.r#west == true && block_state.r#north == true && block_state.r#age == 15 && block_state.r#south == false { return 3676; }
        if block_state.r#south == true && block_state.r#north == false && block_state.r#age == 1 && block_state.r#east == false && block_state.r#west == false && block_state.r#up == true { return 3231; }
        if block_state.r#up == false && block_state.r#east == true && block_state.r#south == true && block_state.r#north == true && block_state.r#age == 6 && block_state.r#west == true { return 3368; }
        if block_state.r#up == false && block_state.r#east == false && block_state.r#south == false && block_state.r#age == 10 && block_state.r#west == false && block_state.r#north == false { return 3525; }
        if block_state.r#north == false && block_state.r#age == 7 && block_state.r#up == true && block_state.r#west == false && block_state.r#south == true && block_state.r#east == false { return 3423; }
        if block_state.r#up == true && block_state.r#east == true && block_state.r#north == true && block_state.r#west == false && block_state.r#south == false && block_state.r#age == 5 { return 3339; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#north == true && block_state.r#age == 4 && block_state.r#south == false && block_state.r#up == false { return 3308; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#up == false && block_state.r#age == 9 && block_state.r#east == false && block_state.r#south == true { return 3488; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#north == true && block_state.r#age == 13 && block_state.r#south == true && block_state.r#up == false { return 3592; }
        if block_state.r#age == 0 && block_state.r#east == true && block_state.r#south == false && block_state.r#north == false && block_state.r#up == false && block_state.r#west == true { return 3188; }
        if block_state.r#age == 4 && block_state.r#east == true && block_state.r#north == false && block_state.r#south == true && block_state.r#up == false && block_state.r#west == false { return 3313; }
        if block_state.r#up == false && block_state.r#west == true && block_state.r#east == false && block_state.r#age == 8 && block_state.r#north == true && block_state.r#south == false { return 3452; }
        if block_state.r#age == 9 && block_state.r#north == true && block_state.r#south == true && block_state.r#east == true && block_state.r#up == true && block_state.r#west == true { return 3462; }
        if block_state.r#age == 5 && block_state.r#up == false && block_state.r#west == true && block_state.r#south == false && block_state.r#north == false && block_state.r#east == true { return 3348; }
        if block_state.r#south == false && block_state.r#north == false && block_state.r#east == true && block_state.r#up == false && block_state.r#age == 8 && block_state.r#west == true { return 3444; }
        if block_state.r#age == 2 && block_state.r#west == true && block_state.r#south == false && block_state.r#up == false && block_state.r#north == false && block_state.r#east == true { return 3252; }
        if block_state.r#age == 4 && block_state.r#up == false && block_state.r#south == false && block_state.r#east == false && block_state.r#north == true && block_state.r#west == true { return 3324; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#age == 1 && block_state.r#south == false && block_state.r#up == true && block_state.r#west == false { return 3219; }
        if block_state.r#age == 0 && block_state.r#up == false && block_state.r#east == false && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true { return 3201; }
        if block_state.r#west == true && block_state.r#age == 8 && block_state.r#south == true && block_state.r#east == false && block_state.r#north == true && block_state.r#up == true { return 3446; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#east == false && block_state.r#up == false && block_state.r#west == true && block_state.r#age == 2 { return 3260; }
        if block_state.r#south == true && block_state.r#up == false && block_state.r#east == false && block_state.r#north == false && block_state.r#age == 14 && block_state.r#west == false { return 3649; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == true && block_state.r#age == 10 { return 3510; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#north == true && block_state.r#west == false && block_state.r#age == 11 && block_state.r#up == false { return 3529; }
        if block_state.r#west == true && block_state.r#south == false && block_state.r#age == 13 && block_state.r#north == false && block_state.r#east == false && block_state.r#up == false { return 3620; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#up == false && block_state.r#west == true && block_state.r#age == 12 { return 3584; }
        if block_state.r#north == true && block_state.r#south == false && block_state.r#west == false && block_state.r#up == false && block_state.r#age == 12 && block_state.r#east == true { return 3565; }
        if block_state.r#age == 6 && block_state.r#east == false && block_state.r#south == true && block_state.r#north == false && block_state.r#west == true && block_state.r#up == true { return 3390; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#up == true && block_state.r#west == true && block_state.r#age == 11 && block_state.r#north == true { return 3546; }
        if block_state.r#age == 12 && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false && block_state.r#up == false && block_state.r#north == true { return 3581; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#up == true && block_state.r#north == true && block_state.r#age == 12 && block_state.r#west == true { return 3562; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#east == true && block_state.r#up == false && block_state.r#age == 0 && block_state.r#south == false { return 3189; }
        if block_state.r#south == true && block_state.r#west == true && block_state.r#north == false && block_state.r#east == false && block_state.r#age == 2 && block_state.r#up == false { return 3264; }
        if block_state.r#age == 7 && block_state.r#north == true && block_state.r#west == false && block_state.r#east == false && block_state.r#south == false && block_state.r#up == true { return 3419; }
        if block_state.r#west == true && block_state.r#age == 3 && block_state.r#up == false && block_state.r#south == true && block_state.r#east == true && block_state.r#north == true { return 3272; }
        if block_state.r#north == false && block_state.r#age == 5 && block_state.r#south == true && block_state.r#west == false && block_state.r#up == true && block_state.r#east == true { return 3343; }
        if block_state.r#south == true && block_state.r#age == 4 && block_state.r#up == false && block_state.r#west == true && block_state.r#east == true && block_state.r#north == false { return 3312; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#up == true && block_state.r#age == 6 && block_state.r#west == true && block_state.r#north == true { return 3366; }
        if block_state.r#up == false && block_state.r#south == false && block_state.r#west == false && block_state.r#age == 6 && block_state.r#east == true && block_state.r#north == true { return 3373; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#up == false && block_state.r#age == 10 && block_state.r#north == true && block_state.r#east == true { return 3501; }
        if block_state.r#west == true && block_state.r#east == true && block_state.r#south == true && block_state.r#age == 10 && block_state.r#up == false && block_state.r#north == false { return 3504; }
        if block_state.r#north == false && block_state.r#up == true && block_state.r#west == true && block_state.r#south == false && block_state.r#age == 2 && block_state.r#east == false { return 3266; }
        if block_state.r#east == false && block_state.r#north == true && block_state.r#up == true && block_state.r#west == false && block_state.r#age == 4 && block_state.r#south == true { return 3319; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#age == 7 && block_state.r#east == true && block_state.r#south == true && block_state.r#up == false { return 3400; }
        if block_state.r#north == true && block_state.r#age == 8 && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false && block_state.r#up == true { return 3451; }
        if block_state.r#north == false && block_state.r#age == 10 && block_state.r#west == false && block_state.r#east == true && block_state.r#south == true && block_state.r#up == false { return 3505; }
        if block_state.r#south == false && block_state.r#up == true && block_state.r#west == false && block_state.r#east == false && block_state.r#age == 10 && block_state.r#north == false { return 3523; }
        if block_state.r#age == 3 && block_state.r#south == false && block_state.r#east == true && block_state.r#west == true && block_state.r#up == true && block_state.r#north == false { return 3282; }
        if block_state.r#age == 8 && block_state.r#south == true && block_state.r#north == false && block_state.r#east == false && block_state.r#up == true && block_state.r#west == false { return 3455; }
        if block_state.r#up == false && block_state.r#north == true && block_state.r#west == false && block_state.r#east == false && block_state.r#south == false && block_state.r#age == 5 { return 3357; }
        if block_state.r#west == false && block_state.r#age == 2 && block_state.r#east == false && block_state.r#south == true && block_state.r#up == true && block_state.r#north == false { return 3263; }
        if block_state.r#age == 4 && block_state.r#west == true && block_state.r#north == true && block_state.r#east == true && block_state.r#up == true && block_state.r#south == true { return 3302; }
        if block_state.r#north == true && block_state.r#up == false && block_state.r#east == true && block_state.r#west == true && block_state.r#age == 11 && block_state.r#south == false { return 3532; }
        if block_state.r#west == true && block_state.r#north == true && block_state.r#south == true && block_state.r#up == true && block_state.r#east == true && block_state.r#age == 13 { return 3590; }
        if block_state.r#south == false && block_state.r#age == 0 && block_state.r#up == true && block_state.r#east == false && block_state.r#north == true && block_state.r#west == false { return 3195; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#up == false && block_state.r#age == 8 && block_state.r#north == true && block_state.r#west == true { return 3448; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#south == false && block_state.r#up == false && block_state.r#age == 0 && block_state.r#west == true { return 3204; }
        if block_state.r#south == true && block_state.r#west == false && block_state.r#east == true && block_state.r#up == false && block_state.r#north == true && block_state.r#age == 8 { return 3433; }
        if block_state.r#south == false && block_state.r#age == 14 && block_state.r#north == true && block_state.r#east == false && block_state.r#west == false && block_state.r#up == true { return 3643; }
        if block_state.r#up == true && block_state.r#east == false && block_state.r#north == false && block_state.r#age == 10 && block_state.r#south == false && block_state.r#west == true { return 3522; }
        if block_state.r#up == true && block_state.r#age == 15 && block_state.r#west == true && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false { return 3666; }
        if block_state.r#age == 4 && block_state.r#north == true && block_state.r#up == false && block_state.r#south == true && block_state.r#west == false && block_state.r#east == false { return 3321; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#up == false && block_state.r#age == 7 && block_state.r#west == false && block_state.r#north == true { return 3417; }
        if block_state.r#up == true && block_state.r#north == true && block_state.r#age == 10 && block_state.r#east == true && block_state.r#south == true && block_state.r#west == false { return 3495; }
        if block_state.r#up == false && block_state.r#east == false && block_state.r#north == false && block_state.r#west == true && block_state.r#age == 11 && block_state.r#south == false { return 3556; }
        if block_state.r#west == false && block_state.r#up == false && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#age == 13 { return 3605; }
        if block_state.r#west == false && block_state.r#age == 2 && block_state.r#south == true && block_state.r#east == false && block_state.r#north == false && block_state.r#up == false { return 3265; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#up == false && block_state.r#south == true && block_state.r#age == 6 && block_state.r#west == true { return 3384; }
        if block_state.r#south == true && block_state.r#east == false && block_state.r#up == false && block_state.r#west == false && block_state.r#age == 11 && block_state.r#north == true { return 3545; }
        if block_state.r#age == 3 && block_state.r#up == false && block_state.r#east == true && block_state.r#west == true && block_state.r#north == false && block_state.r#south == true { return 3280; }
        if block_state.r#west == false && block_state.r#up == false && block_state.r#age == 3 && block_state.r#east == false && block_state.r#north == false && block_state.r#south == false { return 3301; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#south == true && block_state.r#age == 9 && block_state.r#west == false && block_state.r#up == true { return 3471; }
        if block_state.r#age == 2 && block_state.r#north == false && block_state.r#up == true && block_state.r#west == true && block_state.r#south == true && block_state.r#east == false { return 3262; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#south == true && block_state.r#age == 3 && block_state.r#east == false && block_state.r#up == false { return 3297; }
        if block_state.r#north == false && block_state.r#south == true && block_state.r#up == false && block_state.r#age == 6 && block_state.r#west == true && block_state.r#east == false { return 3392; }
        if block_state.r#age == 8 && block_state.r#east == false && block_state.r#north == false && block_state.r#west == true && block_state.r#up == true && block_state.r#south == true { return 3454; }
        if block_state.r#west == false && block_state.r#south == true && block_state.r#up == true && block_state.r#age == 3 && block_state.r#east == false && block_state.r#north == true { return 3287; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#up == true && block_state.r#age == 7 && block_state.r#west == true && block_state.r#south == true { return 3406; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#west == true && block_state.r#age == 9 && block_state.r#south == true && block_state.r#up == false { return 3472; }
        if block_state.r#east == false && block_state.r#west == false && block_state.r#south == true && block_state.r#north == true && block_state.r#up == false && block_state.r#age == 13 { return 3609; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#north == false && block_state.r#east == true && block_state.r#up == true && block_state.r#age == 12 { return 3570; }
        if block_state.r#east == false && block_state.r#south == false && block_state.r#west == true && block_state.r#north == true && block_state.r#age == 1 && block_state.r#up == true { return 3226; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#east == true && block_state.r#north == true && block_state.r#age == 5 && block_state.r#up == true { return 3338; }
        if block_state.r#west == false && block_state.r#north == false && block_state.r#south == true && block_state.r#age == 11 && block_state.r#east == false && block_state.r#up == true { return 3551; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#west == true && block_state.r#east == true && block_state.r#up == true && block_state.r#age == 3 { return 3270; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#up == true && block_state.r#east == false && block_state.r#west == true && block_state.r#age == 12 { return 3586; }
        if block_state.r#east == true && block_state.r#west == false && block_state.r#north == false && block_state.r#age == 11 && block_state.r#south == false && block_state.r#up == false { return 3541; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#up == false && block_state.r#east == false && block_state.r#south == true && block_state.r#age == 1 { return 3224; }
        if block_state.r#up == true && block_state.r#north == true && block_state.r#east == false && block_state.r#age == 2 && block_state.r#south == true && block_state.r#west == true { return 3254; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#north == true && block_state.r#up == true && block_state.r#age == 8 && block_state.r#west == false { return 3431; }
        if block_state.r#age == 12 && block_state.r#north == true && block_state.r#south == true && block_state.r#east == false && block_state.r#west == true && block_state.r#up == false { return 3576; }
        if block_state.r#up == true && block_state.r#west == false && block_state.r#age == 1 && block_state.r#east == true && block_state.r#north == true && block_state.r#south == true { return 3207; }
        if block_state.r#west == false && block_state.r#up == true && block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#age == 13 { return 3615; }
        if block_state.r#south == false && block_state.r#north == true && block_state.r#east == false && block_state.r#up == false && block_state.r#west == false && block_state.r#age == 9 { return 3485; }
        if block_state.r#north == true && block_state.r#age == 14 && block_state.r#east == true && block_state.r#up == true && block_state.r#south == true && block_state.r#west == false { return 3623; }
        if block_state.r#south == false && block_state.r#up == false && block_state.r#east == false && block_state.r#age == 10 && block_state.r#west == true && block_state.r#north == false { return 3524; }
        if block_state.r#up == true && block_state.r#west == false && block_state.r#age == 10 && block_state.r#south == true && block_state.r#north == false && block_state.r#east == false { return 3519; }
        if block_state.r#south == false && block_state.r#west == true && block_state.r#age == 12 && block_state.r#up == false && block_state.r#north == true && block_state.r#east == true { return 3564; }
        if block_state.r#age == 7 && block_state.r#east == true && block_state.r#up == true && block_state.r#north == true && block_state.r#west == true && block_state.r#south == true { return 3398; }
        if block_state.r#age == 5 && block_state.r#south == false && block_state.r#east == true && block_state.r#west == true && block_state.r#up == false && block_state.r#north == true { return 3340; }
        if block_state.r#age == 13 && block_state.r#south == false && block_state.r#up == false && block_state.r#east == false && block_state.r#west == false && block_state.r#north == false { return 3621; }
        if block_state.r#up == true && block_state.r#west == false && block_state.r#age == 0 && block_state.r#north == false && block_state.r#south == false && block_state.r#east == true { return 3187; }
        if block_state.r#west == true && block_state.r#south == true && block_state.r#east == true && block_state.r#north == true && block_state.r#up == false && block_state.r#age == 15 { return 3656; }
        if block_state.r#up == false && block_state.r#age == 3 && block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#west == true { return 3296; }
        if block_state.r#up == false && block_state.r#age == 7 && block_state.r#north == true && block_state.r#east == true && block_state.r#south == false && block_state.r#west == false { return 3405; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#east == false && block_state.r#age == 0 && block_state.r#up == false && block_state.r#west == false { return 3205; }
        if block_state.r#up == true && block_state.r#east == true && block_state.r#age == 2 && block_state.r#north == false && block_state.r#south == true && block_state.r#west == true { return 3246; }
        if block_state.r#up == true && block_state.r#west == true && block_state.r#south == true && block_state.r#age == 15 && block_state.r#east == true && block_state.r#north == false { return 3662; }
        if block_state.r#east == true && block_state.r#up == true && block_state.r#west == false && block_state.r#north == false && block_state.r#age == 1 && block_state.r#south == true { return 3215; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#east == false && block_state.r#up == true && block_state.r#age == 14 && block_state.r#west == false { return 3639; }
        if block_state.r#north == true && block_state.r#up == true && block_state.r#east == true && block_state.r#age == 8 && block_state.r#south == false && block_state.r#west == true { return 3434; }
        if block_state.r#age == 14 && block_state.r#east == false && block_state.r#south == false && block_state.r#up == false && block_state.r#west == true && block_state.r#north == true { return 3644; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#age == 15 && block_state.r#up == false && block_state.r#south == false && block_state.r#north == true { return 3677; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#up == false && block_state.r#north == true && block_state.r#age == 8 && block_state.r#west == true { return 3432; }
        if block_state.r#south == false && block_state.r#west == false && block_state.r#up == true && block_state.r#east == false && block_state.r#age == 5 && block_state.r#north == true { return 3355; }
        if block_state.r#up == true && block_state.r#south == true && block_state.r#north == true && block_state.r#west == false && block_state.r#east == true && block_state.r#age == 3 { return 3271; }
        if block_state.r#up == false && block_state.r#south == false && block_state.r#west == true && block_state.r#age == 7 && block_state.r#east == false && block_state.r#north == true { return 3420; }
        if block_state.r#east == false && block_state.r#age == 7 && block_state.r#south == false && block_state.r#west == false && block_state.r#up == true && block_state.r#north == false { return 3427; }
        if block_state.r#age == 6 && block_state.r#north == false && block_state.r#east == false && block_state.r#south == false && block_state.r#west == false && block_state.r#up == false { return 3397; }
        if block_state.r#east == false && block_state.r#up == false && block_state.r#age == 8 && block_state.r#north == false && block_state.r#west == true && block_state.r#south == false { return 3460; }
        if block_state.r#east == true && block_state.r#west == true && block_state.r#south == true && block_state.r#north == false && block_state.r#age == 3 && block_state.r#up == true { return 3278; }
        if block_state.r#west == false && block_state.r#east == true && block_state.r#south == false && block_state.r#age == 10 && block_state.r#up == true && block_state.r#north == true { return 3499; }
        if block_state.r#west == true && block_state.r#north == false && block_state.r#east == false && block_state.r#age == 0 && block_state.r#south == true && block_state.r#up == true { return 3198; }
        if block_state.r#up == false && block_state.r#north == false && block_state.r#east == false && block_state.r#age == 5 && block_state.r#south == false && block_state.r#west == false { return 3365; }
        if block_state.r#east == false && block_state.r#west == true && block_state.r#north == true && block_state.r#age == 1 && block_state.r#up == false && block_state.r#south == false { return 3228; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#age == 7 && block_state.r#up == false && block_state.r#east == false && block_state.r#west == true { return 3416; }
        if block_state.r#west == false && block_state.r#north == true && block_state.r#east == true && block_state.r#south == true && block_state.r#age == 14 && block_state.r#up == false { return 3625; }
        if block_state.r#east == false && block_state.r#age == 7 && block_state.r#north == true && block_state.r#up == true && block_state.r#south == true && block_state.r#west == true { return 3414; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#south == true && block_state.r#age == 11 && block_state.r#up == false && block_state.r#west == false { return 3537; }
        if block_state.r#up == false && block_state.r#north == true && block_state.r#south == true && block_state.r#east == true && block_state.r#west == false && block_state.r#age == 3 { return 3273; }
        if block_state.r#age == 11 && block_state.r#north == true && block_state.r#east == false && block_state.r#south == true && block_state.r#up == true && block_state.r#west == true { return 3542; }
        if block_state.r#east == true && block_state.r#north == true && block_state.r#south == false && block_state.r#up == false && block_state.r#west == false && block_state.r#age == 13 { return 3597; }
        if block_state.r#north == true && block_state.r#south == true && block_state.r#west == false && block_state.r#up == true && block_state.r#age == 9 && block_state.r#east == true { return 3463; }
        if block_state.r#south == true && block_state.r#north == true && block_state.r#age == 7 && block_state.r#west == false && block_state.r#east == true && block_state.r#up == false { return 3401; }
        if block_state.r#east == true && block_state.r#south == false && block_state.r#up == false && block_state.r#west == true && block_state.r#age == 15 && block_state.r#north == false { return 3668; }
        if block_state.r#west == false && block_state.r#east == true && block_state.r#north == false && block_state.r#age == 9 && block_state.r#south == false && block_state.r#up == false { return 3477; }
        if block_state.r#east == true && block_state.r#age == 10 && block_state.r#west == false && block_state.r#north == false && block_state.r#south == false && block_state.r#up == true { return 3507; }
        if block_state.r#north == true && block_state.r#up == true && block_state.r#east == false && block_state.r#age == 13 && block_state.r#south == true && block_state.r#west == true { return 3606; }
        if block_state.r#south == true && block_state.r#up == false && block_state.r#west == true && block_state.r#east == true && block_state.r#age == 1 && block_state.r#north == true { return 3208; }
        if block_state.r#age == 6 && block_state.r#east == true && block_state.r#west == true && block_state.r#north == false && block_state.r#up == true && block_state.r#south == false { return 3378; }
        if block_state.r#age == 1 && block_state.r#west == true && block_state.r#up == false && block_state.r#east == true && block_state.r#south == false && block_state.r#north == true { return 3212; }
        if block_state.r#south == false && block_state.r#age == 7 && block_state.r#north == true && block_state.r#up == true && block_state.r#east == true && block_state.r#west == true { return 3402; }
        if block_state.r#east == true && block_state.r#north == false && block_state.r#age == 12 && block_state.r#west == false && block_state.r#south == true && block_state.r#up == true { return 3567; }
        if block_state.r#north == false && block_state.r#east == false && block_state.r#up == true && block_state.r#age == 13 && block_state.r#south == false && block_state.r#west == false { return 3619; }
        if block_state.r#age == 15 && block_state.r#south == false && block_state.r#east == true && block_state.r#north == true && block_state.r#up == true && block_state.r#west == false { return 3659; }
        if block_state.r#age == 2 && block_state.r#east == true && block_state.r#north == false && block_state.r#up == false && block_state.r#south == false && block_state.r#west == false { return 3253; }
        if block_state.r#up == true && block_state.r#west == true && block_state.r#age == 4 && block_state.r#east == false && block_state.r#north == false && block_state.r#south == false { return 3330; }
        if block_state.r#up == false && block_state.r#west == false && block_state.r#east == true && block_state.r#north == false && block_state.r#age == 12 && block_state.r#south == true { return 3569; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#north == false && block_state.r#south == true && block_state.r#up == true && block_state.r#age == 12 { return 3583; }
        if block_state.r#up == true && block_state.r#age == 13 && block_state.r#north == false && block_state.r#west == true && block_state.r#east == true && block_state.r#south == true { return 3598; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#south == false && block_state.r#west == false && block_state.r#age == 1 && block_state.r#up == false { return 3213; }
        if block_state.r#age == 6 && block_state.r#east == true && block_state.r#up == true && block_state.r#north == true && block_state.r#south == true && block_state.r#west == false { return 3367; }
        if block_state.r#north == false && block_state.r#east == true && block_state.r#south == true && block_state.r#age == 14 && block_state.r#west == false && block_state.r#up == true { return 3631; }
        if block_state.r#up == false && block_state.r#north == true && block_state.r#age == 2 && block_state.r#west == false && block_state.r#east == true && block_state.r#south == false { return 3245; }
        if block_state.r#north == false && block_state.r#south == false && block_state.r#west == true && block_state.r#east == false && block_state.r#up == false && block_state.r#age == 2 { return 3268; }
        if block_state.r#age == 12 && block_state.r#north == true && block_state.r#south == false && block_state.r#west == true && block_state.r#east == false && block_state.r#up == true { return 3578; }
        if block_state.r#east == true && block_state.r#west == false && block_state.r#south == true && block_state.r#north == true && block_state.r#age == 1 && block_state.r#up == false { return 3209; }
        if block_state.r#up == true && block_state.r#west == false && block_state.r#north == true && block_state.r#south == true && block_state.r#age == 1 && block_state.r#east == false { return 3223; }
        if block_state.r#east == true && block_state.r#south == true && block_state.r#up == false && block_state.r#west == false && block_state.r#age == 6 && block_state.r#north == false { return 3377; }
        if block_state.r#west == false && block_state.r#age == 9 && block_state.r#east == false && block_state.r#north == true && block_state.r#south == true && block_state.r#up == false { return 3481; }
        if block_state.r#age == 6 && block_state.r#east == true && block_state.r#north == false && block_state.r#south == false && block_state.r#up == false && block_state.r#west == true { return 3380; }
        if block_state.r#south == true && block_state.r#up == true && block_state.r#age == 6 && block_state.r#east == false && block_state.r#north == false && block_state.r#west == false { return 3391; }
        if block_state.r#north == true && block_state.r#west == true && block_state.r#south == false && block_state.r#up == false && block_state.r#age == 8 && block_state.r#east == true { return 3436; }
        if block_state.r#west == false && block_state.r#east == false && block_state.r#south == false && block_state.r#age == 10 && block_state.r#north == true && block_state.r#up == true { return 3515; }
        if block_state.r#east == true && block_state.r#age == 12 && block_state.r#south == false && block_state.r#up == false && block_state.r#north == false && block_state.r#west == true { return 3572; }
        if block_state.r#north == true && block_state.r#age == 12 && block_state.r#west == false && block_state.r#east == false && block_state.r#south == false && block_state.r#up == true { return 3579; }
        if block_state.r#north == false && block_state.r#age == 12 && block_state.r#east == false && block_state.r#up == false && block_state.r#west == true && block_state.r#south == false { return 3588; }
        if block_state.r#west == true && block_state.r#age == 13 && block_state.r#north == true && block_state.r#south == false && block_state.r#east == false && block_state.r#up == false { return 3612; }
        if block_state.r#east == true && block_state.r#west == false && block_state.r#north == true && block_state.r#age == 7 && block_state.r#up == true && block_state.r#south == false { return 3403; }
        if block_state.r#north == false && block_state.r#age == 9 && block_state.r#south == true && block_state.r#west == true && block_state.r#east == false && block_state.r#up == true { return 3486; }
        if block_state.r#west == false && block_state.r#age == 8 && block_state.r#south == true && block_state.r#north == false && block_state.r#up == false && block_state.r#east == false { return 3457; }
        if block_state.r#age == 1 && block_state.r#up == false && block_state.r#west == false && block_state.r#north == false && block_state.r#south == true && block_state.r#east == false { return 3233; }
        if block_state.r#north == true && block_state.r#east == false && block_state.r#south == true && block_state.r#age == 5 && block_state.r#west == true && block_state.r#up == false { return 3352; }
        if block_state.r#east == false && block_state.r#north == false && block_state.r#age == 4 && block_state.r#south == true && block_state.r#up == false && block_state.r#west == false { return 3329; }
        if block_state.r#up == false && block_state.r#east == false && block_state.r#west == true && block_state.r#north == true && block_state.r#age == 5 && block_state.r#south == false { return 3356; }
        if block_state.r#age == 1 && block_state.r#up == true && block_state.r#north == true && block_state.r#east == false && block_state.r#west == false && block_state.r#south == false { return 3227; }
        if block_state.r#up == true && block_state.r#age == 3 && block_state.r#south == true && block_state.r#east == false && block_state.r#north == false && block_state.r#west == true { return 3294; }
        if block_state.r#west == false && block_state.r#age == 9 && block_state.r#east == true && block_state.r#up == false && block_state.r#north == true && block_state.r#south == true { return 3465; }
        if block_state.r#south == false && block_state.r#up == true && block_state.r#west == false && block_state.r#age == 12 && block_state.r#east == false && block_state.r#north == false { return 3587; }
        if block_state.r#age == 15 && block_state.r#south == true && block_state.r#east == false && block_state.r#north == false && block_state.r#up == true && block_state.r#west == true { return 3678; }
        if block_state.r#south == false && block_state.r#age == 14 && block_state.r#east == false && block_state.r#up == true && block_state.r#west == false && block_state.r#north == false { return 3651; }
        if block_state.r#north == false && block_state.r#age == 4 && block_state.r#south == false && block_state.r#west == false && block_state.r#east == false && block_state.r#up == false { return 3333; }
        if block_state.r#age == 10 && block_state.r#south == true && block_state.r#west == false && block_state.r#east == false && block_state.r#up == false && block_state.r#north == false { return 3521; }
        if block_state.r#age == 12 && block_state.r#east == true && block_state.r#south == true && block_state.r#up == true && block_state.r#west == false && block_state.r#north == true { return 3559; }
        if block_state.r#east == false && block_state.r#south == true && block_state.r#up == true && block_state.r#age == 4 && block_state.r#west == true && block_state.r#north == false { return 3326; }
        if block_state.r#age == 8 && block_state.r#south == true && block_state.r#north == false && block_state.r#up == false && block_state.r#east == true && block_state.r#west == true { return 3440; }
        if block_state.r#south == true && block_state.r#east == true && block_state.r#north == false && block_state.r#up == false && block_state.r#west == true && block_state.r#age == 14 { return 3632; }
        if block_state.r#north == true && block_state.r#east == true && block_state.r#up == false && block_state.r#west == false && block_state.r#age == 9 && block_state.r#south == false { return 3469; }
        if block_state.r#west == false && block_state.r#age == 0 && block_state.r#east == true && block_state.r#north == true && block_state.r#south == true && block_state.r#up == false { return 3177; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3235 {
            return Some(Fire {
                r#south: false,
                r#age: 1,
                r#up: true,
                r#west: false,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 3470 {
            return Some(Fire {
                r#east: true,
                r#north: false,
                r#up: true,
                r#age: 9,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 3258 {
            return Some(Fire {
                r#east: false,
                r#south: false,
                r#north: true,
                r#age: 2,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 3221 {
            return Some(Fire {
                r#west: false,
                r#east: true,
                r#north: false,
                r#age: 1,
                r#south: false,
                r#up: false,
            });
        }
        if state_id == 3502 {
            return Some(Fire {
                r#north: false,
                r#up: true,
                r#east: true,
                r#south: true,
                r#age: 10,
                r#west: true,
            });
        }
        if state_id == 3630 {
            return Some(Fire {
                r#north: false,
                r#age: 14,
                r#east: true,
                r#up: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 3641 {
            return Some(Fire {
                r#age: 14,
                r#north: true,
                r#east: false,
                r#south: true,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 3300 {
            return Some(Fire {
                r#up: false,
                r#south: false,
                r#east: false,
                r#age: 3,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 3388 {
            return Some(Fire {
                r#up: false,
                r#north: true,
                r#age: 6,
                r#west: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 3646 {
            return Some(Fire {
                r#west: true,
                r#up: true,
                r#age: 14,
                r#south: true,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 3664 {
            return Some(Fire {
                r#south: true,
                r#east: true,
                r#age: 15,
                r#north: false,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 3255 {
            return Some(Fire {
                r#age: 2,
                r#up: true,
                r#north: true,
                r#east: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 3354 {
            return Some(Fire {
                r#age: 5,
                r#up: true,
                r#east: false,
                r#south: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 3217 {
            return Some(Fire {
                r#west: false,
                r#up: false,
                r#north: false,
                r#south: true,
                r#age: 1,
                r#east: true,
            });
        }
        if state_id == 3214 {
            return Some(Fire {
                r#age: 1,
                r#up: true,
                r#east: true,
                r#north: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 3385 {
            return Some(Fire {
                r#up: false,
                r#north: true,
                r#east: false,
                r#south: true,
                r#west: false,
                r#age: 6,
            });
        }
        if state_id == 3185 {
            return Some(Fire {
                r#east: true,
                r#up: false,
                r#west: false,
                r#age: 0,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 3684 {
            return Some(Fire {
                r#north: false,
                r#age: 15,
                r#up: false,
                r#west: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 3337 {
            return Some(Fire {
                r#east: true,
                r#west: false,
                r#up: false,
                r#north: true,
                r#south: true,
                r#age: 5,
            });
        }
        if state_id == 3484 {
            return Some(Fire {
                r#east: false,
                r#up: false,
                r#west: true,
                r#south: false,
                r#north: true,
                r#age: 9,
            });
        }
        if state_id == 3638 {
            return Some(Fire {
                r#east: false,
                r#age: 14,
                r#south: true,
                r#up: true,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 3415 {
            return Some(Fire {
                r#age: 7,
                r#north: true,
                r#west: false,
                r#east: false,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 3314 {
            return Some(Fire {
                r#west: true,
                r#south: false,
                r#up: true,
                r#age: 4,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 3376 {
            return Some(Fire {
                r#north: false,
                r#up: false,
                r#east: true,
                r#age: 6,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 3275 {
            return Some(Fire {
                r#age: 3,
                r#north: true,
                r#east: true,
                r#south: false,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3289 {
            return Some(Fire {
                r#south: true,
                r#age: 3,
                r#east: false,
                r#north: true,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 3483 {
            return Some(Fire {
                r#south: false,
                r#east: false,
                r#west: false,
                r#north: true,
                r#up: true,
                r#age: 9,
            });
        }
        if state_id == 3561 {
            return Some(Fire {
                r#age: 12,
                r#south: true,
                r#up: false,
                r#east: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 3315 {
            return Some(Fire {
                r#north: false,
                r#west: false,
                r#age: 4,
                r#east: true,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 3591 {
            return Some(Fire {
                r#age: 13,
                r#east: true,
                r#south: true,
                r#up: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 3218 {
            return Some(Fire {
                r#east: true,
                r#north: false,
                r#west: true,
                r#south: false,
                r#age: 1,
                r#up: true,
            });
        }
        if state_id == 3342 {
            return Some(Fire {
                r#up: true,
                r#south: true,
                r#east: true,
                r#west: true,
                r#north: false,
                r#age: 5,
            });
        }
        if state_id == 3216 {
            return Some(Fire {
                r#west: true,
                r#north: false,
                r#south: true,
                r#east: true,
                r#up: false,
                r#age: 1,
            });
        }
        if state_id == 3534 {
            return Some(Fire {
                r#age: 11,
                r#up: true,
                r#west: true,
                r#east: true,
                r#south: true,
                r#north: false,
            });
        }
        if state_id == 3535 {
            return Some(Fire {
                r#south: true,
                r#age: 11,
                r#up: true,
                r#west: false,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 3389 {
            return Some(Fire {
                r#east: false,
                r#up: false,
                r#south: false,
                r#age: 6,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 3467 {
            return Some(Fire {
                r#south: false,
                r#north: true,
                r#age: 9,
                r#east: true,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3574 {
            return Some(Fire {
                r#up: true,
                r#west: true,
                r#south: true,
                r#east: false,
                r#age: 12,
                r#north: true,
            });
        }
        if state_id == 3557 {
            return Some(Fire {
                r#age: 11,
                r#south: false,
                r#up: false,
                r#east: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 3600 {
            return Some(Fire {
                r#south: true,
                r#up: false,
                r#north: false,
                r#west: true,
                r#east: true,
                r#age: 13,
            });
        }
        if state_id == 3516 {
            return Some(Fire {
                r#age: 10,
                r#south: false,
                r#east: false,
                r#west: true,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 3538 {
            return Some(Fire {
                r#north: false,
                r#west: true,
                r#age: 11,
                r#east: true,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 3554 {
            return Some(Fire {
                r#west: true,
                r#age: 11,
                r#south: false,
                r#east: false,
                r#north: false,
                r#up: true,
            });
        }
        if state_id == 3306 {
            return Some(Fire {
                r#age: 4,
                r#east: true,
                r#north: true,
                r#south: false,
                r#west: true,
                r#up: true,
            });
        }
        if state_id == 3571 {
            return Some(Fire {
                r#east: true,
                r#up: true,
                r#age: 12,
                r#north: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 3618 {
            return Some(Fire {
                r#age: 13,
                r#east: false,
                r#south: false,
                r#up: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 3370 {
            return Some(Fire {
                r#age: 6,
                r#east: true,
                r#up: true,
                r#south: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 3650 {
            return Some(Fire {
                r#age: 14,
                r#north: false,
                r#up: true,
                r#west: true,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 3500 {
            return Some(Fire {
                r#age: 10,
                r#west: true,
                r#up: false,
                r#north: true,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 3347 {
            return Some(Fire {
                r#up: true,
                r#west: false,
                r#north: false,
                r#south: false,
                r#east: true,
                r#age: 5,
            });
        }
        if state_id == 3309 {
            return Some(Fire {
                r#up: false,
                r#east: true,
                r#north: true,
                r#west: false,
                r#south: false,
                r#age: 4,
            });
        }
        if state_id == 3549 {
            return Some(Fire {
                r#up: false,
                r#east: false,
                r#age: 11,
                r#south: false,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 3555 {
            return Some(Fire {
                r#up: true,
                r#north: false,
                r#age: 11,
                r#west: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 3653 {
            return Some(Fire {
                r#west: false,
                r#age: 14,
                r#east: false,
                r#south: false,
                r#up: false,
                r#north: false,
            });
        }
        if state_id == 3344 {
            return Some(Fire {
                r#south: true,
                r#age: 5,
                r#north: false,
                r#west: true,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 3334 {
            return Some(Fire {
                r#east: true,
                r#south: true,
                r#up: true,
                r#west: true,
                r#north: true,
                r#age: 5,
            });
        }
        if state_id == 3179 {
            return Some(Fire {
                r#age: 0,
                r#west: false,
                r#east: true,
                r#south: false,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 3425 {
            return Some(Fire {
                r#south: true,
                r#up: false,
                r#age: 7,
                r#north: false,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 3660 {
            return Some(Fire {
                r#north: true,
                r#south: false,
                r#age: 15,
                r#west: true,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 3404 {
            return Some(Fire {
                r#age: 7,
                r#up: false,
                r#north: true,
                r#east: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 3225 {
            return Some(Fire {
                r#age: 1,
                r#north: true,
                r#west: false,
                r#south: true,
                r#east: false,
                r#up: false,
            });
        }
        if state_id == 3230 {
            return Some(Fire {
                r#age: 1,
                r#east: false,
                r#north: false,
                r#south: true,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 3237 {
            return Some(Fire {
                r#age: 1,
                r#north: false,
                r#east: false,
                r#south: false,
                r#west: false,
                r#up: false,
            });
        }
        if state_id == 3675 {
            return Some(Fire {
                r#up: true,
                r#south: false,
                r#west: false,
                r#east: false,
                r#age: 15,
                r#north: true,
            });
        }
        if state_id == 3363 {
            return Some(Fire {
                r#up: true,
                r#age: 5,
                r#east: false,
                r#west: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 3335 {
            return Some(Fire {
                r#north: true,
                r#up: true,
                r#south: true,
                r#age: 5,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 3274 {
            return Some(Fire {
                r#east: true,
                r#north: true,
                r#south: false,
                r#up: true,
                r#west: true,
                r#age: 3,
            });
        }
        if state_id == 3553 {
            return Some(Fire {
                r#west: false,
                r#age: 11,
                r#north: false,
                r#east: false,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 3445 {
            return Some(Fire {
                r#south: false,
                r#age: 8,
                r#north: false,
                r#up: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 3180 {
            return Some(Fire {
                r#south: false,
                r#north: true,
                r#east: true,
                r#up: false,
                r#age: 0,
                r#west: true,
            });
        }
        if state_id == 3290 {
            return Some(Fire {
                r#up: true,
                r#west: true,
                r#age: 3,
                r#east: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 3362 {
            return Some(Fire {
                r#south: false,
                r#west: true,
                r#age: 5,
                r#east: false,
                r#north: false,
                r#up: true,
            });
        }
        if state_id == 3443 {
            return Some(Fire {
                r#age: 8,
                r#east: true,
                r#north: false,
                r#south: false,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3655 {
            return Some(Fire {
                r#west: false,
                r#east: true,
                r#south: true,
                r#north: true,
                r#up: true,
                r#age: 15,
            });
        }
        if state_id == 3197 {
            return Some(Fire {
                r#east: false,
                r#north: true,
                r#south: false,
                r#age: 0,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 3202 {
            return Some(Fire {
                r#age: 0,
                r#south: false,
                r#west: true,
                r#up: true,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 3210 {
            return Some(Fire {
                r#west: true,
                r#east: true,
                r#south: false,
                r#age: 1,
                r#up: true,
                r#north: true,
            });
        }
        if state_id == 3396 {
            return Some(Fire {
                r#south: false,
                r#north: false,
                r#west: true,
                r#age: 6,
                r#east: false,
                r#up: false,
            });
        }
        if state_id == 3241 {
            return Some(Fire {
                r#north: true,
                r#up: false,
                r#west: false,
                r#age: 2,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 3595 {
            return Some(Fire {
                r#south: false,
                r#east: true,
                r#age: 13,
                r#north: true,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3493 {
            return Some(Fire {
                r#south: false,
                r#east: false,
                r#west: false,
                r#age: 9,
                r#north: false,
                r#up: false,
            });
        }
        if state_id == 3317 {
            return Some(Fire {
                r#north: false,
                r#up: false,
                r#age: 4,
                r#east: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 3610 {
            return Some(Fire {
                r#south: false,
                r#up: true,
                r#north: true,
                r#east: false,
                r#west: true,
                r#age: 13,
            });
        }
        if state_id == 3407 {
            return Some(Fire {
                r#south: true,
                r#up: true,
                r#north: false,
                r#west: false,
                r#age: 7,
                r#east: true,
            });
        }
        if state_id == 3261 {
            return Some(Fire {
                r#south: false,
                r#east: false,
                r#up: false,
                r#age: 2,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 3520 {
            return Some(Fire {
                r#age: 10,
                r#east: false,
                r#up: false,
                r#north: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 3517 {
            return Some(Fire {
                r#west: false,
                r#up: false,
                r#age: 10,
                r#north: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 3568 {
            return Some(Fire {
                r#age: 12,
                r#north: false,
                r#south: true,
                r#east: true,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 3206 {
            return Some(Fire {
                r#north: true,
                r#age: 1,
                r#west: true,
                r#east: true,
                r#up: true,
                r#south: true,
            });
        }
        if state_id == 3394 {
            return Some(Fire {
                r#age: 6,
                r#north: false,
                r#west: true,
                r#east: false,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 3424 {
            return Some(Fire {
                r#age: 7,
                r#south: true,
                r#east: false,
                r#up: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 3622 {
            return Some(Fire {
                r#east: true,
                r#north: true,
                r#age: 14,
                r#south: true,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 3232 {
            return Some(Fire {
                r#north: false,
                r#east: false,
                r#age: 1,
                r#south: true,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 3475 {
            return Some(Fire {
                r#age: 9,
                r#south: false,
                r#up: true,
                r#north: false,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 3222 {
            return Some(Fire {
                r#age: 1,
                r#north: true,
                r#east: false,
                r#west: true,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 3242 {
            return Some(Fire {
                r#west: true,
                r#up: true,
                r#south: false,
                r#east: true,
                r#age: 2,
                r#north: true,
            });
        }
        if state_id == 3411 {
            return Some(Fire {
                r#north: false,
                r#south: false,
                r#east: true,
                r#age: 7,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3489 {
            return Some(Fire {
                r#west: false,
                r#north: false,
                r#south: true,
                r#east: false,
                r#up: false,
                r#age: 9,
            });
        }
        if state_id == 3518 {
            return Some(Fire {
                r#west: true,
                r#up: true,
                r#age: 10,
                r#north: false,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 3184 {
            return Some(Fire {
                r#east: true,
                r#south: true,
                r#age: 0,
                r#north: false,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 3633 {
            return Some(Fire {
                r#west: false,
                r#age: 14,
                r#east: true,
                r#south: true,
                r#north: false,
                r#up: false,
            });
        }
        if state_id == 3281 {
            return Some(Fire {
                r#age: 3,
                r#north: false,
                r#west: false,
                r#south: true,
                r#up: false,
                r#east: true,
            });
        }
        if state_id == 3316 {
            return Some(Fire {
                r#east: true,
                r#south: false,
                r#up: false,
                r#west: true,
                r#north: false,
                r#age: 4,
            });
        }
        if state_id == 3607 {
            return Some(Fire {
                r#south: true,
                r#north: true,
                r#west: false,
                r#up: true,
                r#east: false,
                r#age: 13,
            });
        }
        if state_id == 3514 {
            return Some(Fire {
                r#age: 10,
                r#north: true,
                r#east: false,
                r#south: false,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 3393 {
            return Some(Fire {
                r#north: false,
                r#age: 6,
                r#east: false,
                r#up: false,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 3673 {
            return Some(Fire {
                r#up: false,
                r#age: 15,
                r#north: true,
                r#south: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 3418 {
            return Some(Fire {
                r#south: false,
                r#up: true,
                r#west: true,
                r#east: false,
                r#age: 7,
                r#north: true,
            });
        }
        if state_id == 3291 {
            return Some(Fire {
                r#age: 3,
                r#north: true,
                r#west: false,
                r#south: false,
                r#east: false,
                r#up: true,
            });
        }
        if state_id == 3611 {
            return Some(Fire {
                r#age: 13,
                r#south: false,
                r#north: true,
                r#up: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 3395 {
            return Some(Fire {
                r#east: false,
                r#age: 6,
                r#up: true,
                r#north: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 3479 {
            return Some(Fire {
                r#south: true,
                r#up: true,
                r#west: false,
                r#north: true,
                r#east: false,
                r#age: 9,
            });
        }
        if state_id == 3236 {
            return Some(Fire {
                r#west: true,
                r#age: 1,
                r#up: false,
                r#north: false,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 3577 {
            return Some(Fire {
                r#east: false,
                r#up: false,
                r#age: 12,
                r#west: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 3364 {
            return Some(Fire {
                r#east: false,
                r#up: false,
                r#west: true,
                r#north: false,
                r#age: 5,
                r#south: false,
            });
        }
        if state_id == 3527 {
            return Some(Fire {
                r#west: false,
                r#north: true,
                r#south: true,
                r#up: true,
                r#east: true,
                r#age: 11,
            });
        }
        if state_id == 3531 {
            return Some(Fire {
                r#age: 11,
                r#east: true,
                r#north: true,
                r#south: false,
                r#west: false,
                r#up: true,
            });
        }
        if state_id == 3429 {
            return Some(Fire {
                r#west: false,
                r#up: false,
                r#east: false,
                r#age: 7,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 3243 {
            return Some(Fire {
                r#east: true,
                r#age: 2,
                r#north: true,
                r#south: false,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3634 {
            return Some(Fire {
                r#age: 14,
                r#east: true,
                r#south: false,
                r#up: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 3186 {
            return Some(Fire {
                r#south: false,
                r#east: true,
                r#age: 0,
                r#up: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 3466 {
            return Some(Fire {
                r#south: false,
                r#up: true,
                r#west: true,
                r#age: 9,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 3327 {
            return Some(Fire {
                r#east: false,
                r#south: true,
                r#up: true,
                r#west: false,
                r#age: 4,
                r#north: false,
            });
        }
        if state_id == 3181 {
            return Some(Fire {
                r#south: false,
                r#north: true,
                r#up: false,
                r#west: false,
                r#age: 0,
                r#east: true,
            });
        }
        if state_id == 3637 {
            return Some(Fire {
                r#south: false,
                r#age: 14,
                r#up: false,
                r#east: true,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 3196 {
            return Some(Fire {
                r#south: false,
                r#up: false,
                r#west: true,
                r#east: false,
                r#age: 0,
                r#north: true,
            });
        }
        if state_id == 3628 {
            return Some(Fire {
                r#west: true,
                r#age: 14,
                r#north: true,
                r#east: true,
                r#south: false,
                r#up: false,
            });
        }
        if state_id == 3437 {
            return Some(Fire {
                r#east: true,
                r#up: false,
                r#west: false,
                r#age: 8,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 3360 {
            return Some(Fire {
                r#east: false,
                r#south: true,
                r#north: false,
                r#west: true,
                r#up: false,
                r#age: 5,
            });
        }
        if state_id == 3482 {
            return Some(Fire {
                r#south: false,
                r#age: 9,
                r#east: false,
                r#up: true,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 3593 {
            return Some(Fire {
                r#west: false,
                r#east: true,
                r#south: true,
                r#age: 13,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 3613 {
            return Some(Fire {
                r#south: false,
                r#east: false,
                r#north: true,
                r#age: 13,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 3490 {
            return Some(Fire {
                r#north: false,
                r#up: true,
                r#west: true,
                r#age: 9,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 3279 {
            return Some(Fire {
                r#south: true,
                r#east: true,
                r#age: 3,
                r#up: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 3648 {
            return Some(Fire {
                r#east: false,
                r#south: true,
                r#up: false,
                r#west: true,
                r#north: false,
                r#age: 14,
            });
        }
        if state_id == 3447 {
            return Some(Fire {
                r#east: false,
                r#age: 8,
                r#south: true,
                r#north: true,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3220 {
            return Some(Fire {
                r#north: false,
                r#age: 1,
                r#east: true,
                r#south: false,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 3663 {
            return Some(Fire {
                r#south: true,
                r#up: true,
                r#west: false,
                r#east: true,
                r#north: false,
                r#age: 15,
            });
        }
        if state_id == 3670 {
            return Some(Fire {
                r#up: true,
                r#west: true,
                r#north: true,
                r#age: 15,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 3543 {
            return Some(Fire {
                r#age: 11,
                r#north: true,
                r#south: true,
                r#up: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 3602 {
            return Some(Fire {
                r#age: 13,
                r#south: false,
                r#up: true,
                r#west: true,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 3442 {
            return Some(Fire {
                r#up: true,
                r#west: true,
                r#south: false,
                r#east: true,
                r#north: false,
                r#age: 8,
            });
        }
        if state_id == 3244 {
            return Some(Fire {
                r#south: false,
                r#west: true,
                r#east: true,
                r#up: false,
                r#age: 2,
                r#north: true,
            });
        }
        if state_id == 3307 {
            return Some(Fire {
                r#west: false,
                r#south: false,
                r#east: true,
                r#age: 4,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 3318 {
            return Some(Fire {
                r#east: false,
                r#age: 4,
                r#up: true,
                r#north: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 3369 {
            return Some(Fire {
                r#west: false,
                r#up: false,
                r#south: true,
                r#north: true,
                r#age: 6,
                r#east: true,
            });
        }
        if state_id == 3426 {
            return Some(Fire {
                r#south: false,
                r#east: false,
                r#up: true,
                r#west: true,
                r#north: false,
                r#age: 7,
            });
        }
        if state_id == 3640 {
            return Some(Fire {
                r#west: true,
                r#east: false,
                r#south: true,
                r#north: true,
                r#age: 14,
                r#up: false,
            });
        }
        if state_id == 3276 {
            return Some(Fire {
                r#west: true,
                r#up: false,
                r#south: false,
                r#east: true,
                r#age: 3,
                r#north: true,
            });
        }
        if state_id == 3349 {
            return Some(Fire {
                r#south: false,
                r#west: false,
                r#up: false,
                r#age: 5,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 3528 {
            return Some(Fire {
                r#west: true,
                r#south: true,
                r#age: 11,
                r#up: false,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 3311 {
            return Some(Fire {
                r#west: false,
                r#north: false,
                r#east: true,
                r#age: 4,
                r#up: true,
                r#south: true,
            });
        }
        if state_id == 3548 {
            return Some(Fire {
                r#up: false,
                r#north: true,
                r#south: false,
                r#east: false,
                r#west: true,
                r#age: 11,
            });
        }
        if state_id == 3513 {
            return Some(Fire {
                r#age: 10,
                r#east: false,
                r#north: true,
                r#south: true,
                r#west: false,
                r#up: false,
            });
        }
        if state_id == 3256 {
            return Some(Fire {
                r#west: true,
                r#north: true,
                r#up: false,
                r#south: true,
                r#east: false,
                r#age: 2,
            });
        }
        if state_id == 3558 {
            return Some(Fire {
                r#west: true,
                r#north: true,
                r#age: 12,
                r#south: true,
                r#up: true,
                r#east: true,
            });
        }
        if state_id == 3372 {
            return Some(Fire {
                r#north: true,
                r#west: true,
                r#south: false,
                r#east: true,
                r#age: 6,
                r#up: false,
            });
        }
        if state_id == 3491 {
            return Some(Fire {
                r#south: false,
                r#age: 9,
                r#north: false,
                r#up: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 3320 {
            return Some(Fire {
                r#age: 4,
                r#west: true,
                r#north: true,
                r#east: false,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 3229 {
            return Some(Fire {
                r#age: 1,
                r#east: false,
                r#south: false,
                r#west: false,
                r#up: false,
                r#north: true,
            });
        }
        if state_id == 3190 {
            return Some(Fire {
                r#age: 0,
                r#north: true,
                r#south: true,
                r#east: false,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 3458 {
            return Some(Fire {
                r#south: false,
                r#west: true,
                r#north: false,
                r#east: false,
                r#age: 8,
                r#up: true,
            });
        }
        if state_id == 3539 {
            return Some(Fire {
                r#west: false,
                r#up: true,
                r#east: true,
                r#age: 11,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 3353 {
            return Some(Fire {
                r#up: false,
                r#south: true,
                r#north: true,
                r#west: false,
                r#age: 5,
                r#east: false,
            });
        }
        if state_id == 3683 {
            return Some(Fire {
                r#up: true,
                r#west: false,
                r#age: 15,
                r#north: false,
                r#east: false,
                r#south: false,
            });
        }
        if state_id == 3305 {
            return Some(Fire {
                r#south: true,
                r#up: false,
                r#age: 4,
                r#north: true,
                r#west: false,
                r#east: true,
            });
        }
        if state_id == 3453 {
            return Some(Fire {
                r#south: false,
                r#west: false,
                r#up: false,
                r#east: false,
                r#north: true,
                r#age: 8,
            });
        }
        if state_id == 3594 {
            return Some(Fire {
                r#up: true,
                r#south: false,
                r#north: true,
                r#age: 13,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 3249 {
            return Some(Fire {
                r#east: true,
                r#age: 2,
                r#north: false,
                r#south: true,
                r#west: false,
                r#up: false,
            });
        }
        if state_id == 3174 {
            return Some(Fire {
                r#age: 0,
                r#east: true,
                r#south: true,
                r#west: true,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 3284 {
            return Some(Fire {
                r#west: true,
                r#north: false,
                r#age: 3,
                r#east: true,
                r#up: false,
                r#south: false,
            });
        }
        if state_id == 3310 {
            return Some(Fire {
                r#age: 4,
                r#south: true,
                r#west: true,
                r#north: false,
                r#up: true,
                r#east: true,
            });
        }
        if state_id == 3582 {
            return Some(Fire {
                r#north: false,
                r#west: true,
                r#south: true,
                r#east: false,
                r#up: true,
                r#age: 12,
            });
        }
        if state_id == 3182 {
            return Some(Fire {
                r#south: true,
                r#west: true,
                r#north: false,
                r#age: 0,
                r#up: true,
                r#east: true,
            });
        }
        if state_id == 3487 {
            return Some(Fire {
                r#up: true,
                r#north: false,
                r#age: 9,
                r#south: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 3325 {
            return Some(Fire {
                r#east: false,
                r#north: true,
                r#south: false,
                r#up: false,
                r#age: 4,
                r#west: false,
            });
        }
        if state_id == 3193 {
            return Some(Fire {
                r#west: false,
                r#east: false,
                r#north: true,
                r#up: false,
                r#age: 0,
                r#south: true,
            });
        }
        if state_id == 3359 {
            return Some(Fire {
                r#west: false,
                r#north: false,
                r#south: true,
                r#up: true,
                r#age: 5,
                r#east: false,
            });
        }
        if state_id == 3503 {
            return Some(Fire {
                r#north: false,
                r#age: 10,
                r#up: true,
                r#west: false,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 3234 {
            return Some(Fire {
                r#age: 1,
                r#east: false,
                r#south: false,
                r#up: true,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 3298 {
            return Some(Fire {
                r#up: true,
                r#north: false,
                r#west: true,
                r#age: 3,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 3540 {
            return Some(Fire {
                r#west: true,
                r#up: false,
                r#age: 11,
                r#east: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 3617 {
            return Some(Fire {
                r#south: true,
                r#age: 13,
                r#west: false,
                r#north: false,
                r#east: false,
                r#up: false,
            });
        }
        if state_id == 3508 {
            return Some(Fire {
                r#up: false,
                r#age: 10,
                r#east: true,
                r#north: false,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 3259 {
            return Some(Fire {
                r#age: 2,
                r#up: true,
                r#east: false,
                r#west: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 3323 {
            return Some(Fire {
                r#age: 4,
                r#up: true,
                r#west: false,
                r#east: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 3332 {
            return Some(Fire {
                r#north: false,
                r#west: true,
                r#south: false,
                r#up: false,
                r#age: 4,
                r#east: false,
            });
        }
        if state_id == 3175 {
            return Some(Fire {
                r#south: true,
                r#up: true,
                r#west: false,
                r#east: true,
                r#north: true,
                r#age: 0,
            });
        }
        if state_id == 3552 {
            return Some(Fire {
                r#age: 11,
                r#east: false,
                r#west: true,
                r#south: true,
                r#up: false,
                r#north: false,
            });
        }
        if state_id == 3293 {
            return Some(Fire {
                r#up: false,
                r#age: 3,
                r#west: false,
                r#north: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 3497 {
            return Some(Fire {
                r#north: true,
                r#up: false,
                r#age: 10,
                r#south: true,
                r#east: true,
                r#west: false,
            });
        }
        if state_id == 3346 {
            return Some(Fire {
                r#up: true,
                r#age: 5,
                r#north: false,
                r#east: true,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 3573 {
            return Some(Fire {
                r#south: false,
                r#north: false,
                r#east: true,
                r#age: 12,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 3604 {
            return Some(Fire {
                r#up: false,
                r#east: true,
                r#north: false,
                r#west: true,
                r#south: false,
                r#age: 13,
            });
        }
        if state_id == 3200 {
            return Some(Fire {
                r#age: 0,
                r#east: false,
                r#north: false,
                r#up: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 3430 {
            return Some(Fire {
                r#west: true,
                r#south: true,
                r#east: true,
                r#age: 8,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 3283 {
            return Some(Fire {
                r#north: false,
                r#east: true,
                r#age: 3,
                r#up: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 3601 {
            return Some(Fire {
                r#age: 13,
                r#east: true,
                r#up: false,
                r#west: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 3328 {
            return Some(Fire {
                r#north: false,
                r#south: true,
                r#up: false,
                r#west: true,
                r#age: 4,
                r#east: false,
            });
        }
        if state_id == 3285 {
            return Some(Fire {
                r#up: false,
                r#east: true,
                r#age: 3,
                r#south: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 3509 {
            return Some(Fire {
                r#west: false,
                r#east: true,
                r#south: false,
                r#north: false,
                r#age: 10,
                r#up: false,
            });
        }
        if state_id == 3371 {
            return Some(Fire {
                r#age: 6,
                r#up: true,
                r#north: true,
                r#west: false,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 3629 {
            return Some(Fire {
                r#south: false,
                r#east: true,
                r#north: true,
                r#up: false,
                r#age: 14,
                r#west: false,
            });
        }
        if state_id == 3680 {
            return Some(Fire {
                r#south: true,
                r#east: false,
                r#up: false,
                r#west: true,
                r#north: false,
                r#age: 15,
            });
        }
        if state_id == 3379 {
            return Some(Fire {
                r#east: true,
                r#age: 6,
                r#west: false,
                r#south: false,
                r#north: false,
                r#up: true,
            });
        }
        if state_id == 3382 {
            return Some(Fire {
                r#west: true,
                r#north: true,
                r#age: 6,
                r#east: false,
                r#up: true,
                r#south: true,
            });
        }
        if state_id == 3645 {
            return Some(Fire {
                r#south: false,
                r#east: false,
                r#age: 14,
                r#up: false,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 3383 {
            return Some(Fire {
                r#up: true,
                r#east: false,
                r#south: true,
                r#age: 6,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 3358 {
            return Some(Fire {
                r#age: 5,
                r#north: false,
                r#south: true,
                r#up: true,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 3473 {
            return Some(Fire {
                r#south: true,
                r#age: 9,
                r#north: false,
                r#east: true,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 3409 {
            return Some(Fire {
                r#up: false,
                r#south: true,
                r#east: true,
                r#north: false,
                r#age: 7,
                r#west: false,
            });
        }
        if state_id == 3267 {
            return Some(Fire {
                r#north: false,
                r#south: false,
                r#up: true,
                r#east: false,
                r#age: 2,
                r#west: false,
            });
        }
        if state_id == 3682 {
            return Some(Fire {
                r#north: false,
                r#up: true,
                r#age: 15,
                r#west: true,
                r#south: false,
                r#east: false,
            });
        }
        if state_id == 3526 {
            return Some(Fire {
                r#west: true,
                r#east: true,
                r#age: 11,
                r#south: true,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 3183 {
            return Some(Fire {
                r#south: true,
                r#age: 0,
                r#east: true,
                r#north: false,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3635 {
            return Some(Fire {
                r#age: 14,
                r#north: false,
                r#west: false,
                r#east: true,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 3672 {
            return Some(Fire {
                r#south: true,
                r#north: true,
                r#age: 15,
                r#up: false,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 3277 {
            return Some(Fire {
                r#south: false,
                r#up: false,
                r#west: false,
                r#north: true,
                r#age: 3,
                r#east: true,
            });
        }
        if state_id == 3550 {
            return Some(Fire {
                r#west: true,
                r#age: 11,
                r#east: false,
                r#south: true,
                r#north: false,
                r#up: true,
            });
        }
        if state_id == 3603 {
            return Some(Fire {
                r#west: false,
                r#south: false,
                r#age: 13,
                r#east: true,
                r#north: false,
                r#up: true,
            });
        }
        if state_id == 3671 {
            return Some(Fire {
                r#up: true,
                r#north: true,
                r#west: false,
                r#east: false,
                r#south: true,
                r#age: 15,
            });
        }
        if state_id == 3626 {
            return Some(Fire {
                r#up: true,
                r#east: true,
                r#age: 14,
                r#south: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 3211 {
            return Some(Fire {
                r#east: true,
                r#up: true,
                r#west: false,
                r#north: true,
                r#south: false,
                r#age: 1,
            });
        }
        if state_id == 3292 {
            return Some(Fire {
                r#up: false,
                r#west: true,
                r#age: 3,
                r#east: false,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 3596 {
            return Some(Fire {
                r#west: true,
                r#age: 13,
                r#south: false,
                r#east: true,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 3665 {
            return Some(Fire {
                r#south: true,
                r#east: true,
                r#north: false,
                r#west: false,
                r#up: false,
                r#age: 15,
            });
        }
        if state_id == 3456 {
            return Some(Fire {
                r#up: false,
                r#age: 8,
                r#east: false,
                r#north: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 3512 {
            return Some(Fire {
                r#up: false,
                r#east: false,
                r#south: true,
                r#west: true,
                r#age: 10,
                r#north: true,
            });
        }
        if state_id == 3585 {
            return Some(Fire {
                r#east: false,
                r#north: false,
                r#south: true,
                r#up: false,
                r#west: false,
                r#age: 12,
            });
        }
        if state_id == 3269 {
            return Some(Fire {
                r#south: false,
                r#up: false,
                r#east: false,
                r#west: false,
                r#age: 2,
                r#north: false,
            });
        }
        if state_id == 3560 {
            return Some(Fire {
                r#south: true,
                r#east: true,
                r#up: false,
                r#west: true,
                r#age: 12,
                r#north: true,
            });
        }
        if state_id == 3667 {
            return Some(Fire {
                r#age: 15,
                r#up: true,
                r#north: false,
                r#east: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 3681 {
            return Some(Fire {
                r#west: false,
                r#south: true,
                r#north: false,
                r#age: 15,
                r#east: false,
                r#up: false,
            });
        }
        if state_id == 3636 {
            return Some(Fire {
                r#south: false,
                r#up: false,
                r#age: 14,
                r#west: true,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 3657 {
            return Some(Fire {
                r#west: false,
                r#north: true,
                r#south: true,
                r#age: 15,
                r#up: false,
                r#east: true,
            });
        }
        if state_id == 3492 {
            return Some(Fire {
                r#north: false,
                r#east: false,
                r#south: false,
                r#age: 9,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 3178 {
            return Some(Fire {
                r#age: 0,
                r#east: true,
                r#south: false,
                r#north: true,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 3464 {
            return Some(Fire {
                r#south: true,
                r#up: false,
                r#west: true,
                r#age: 9,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 3386 {
            return Some(Fire {
                r#west: true,
                r#age: 6,
                r#east: false,
                r#north: true,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 3506 {
            return Some(Fire {
                r#east: true,
                r#north: false,
                r#up: true,
                r#west: true,
                r#age: 10,
                r#south: false,
            });
        }
        if state_id == 3412 {
            return Some(Fire {
                r#east: true,
                r#south: false,
                r#up: false,
                r#west: true,
                r#age: 7,
                r#north: false,
            });
        }
        if state_id == 3439 {
            return Some(Fire {
                r#east: true,
                r#up: true,
                r#north: false,
                r#west: false,
                r#south: true,
                r#age: 8,
            });
        }
        if state_id == 3441 {
            return Some(Fire {
                r#age: 8,
                r#south: true,
                r#north: false,
                r#east: true,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 3544 {
            return Some(Fire {
                r#south: true,
                r#up: false,
                r#west: true,
                r#east: false,
                r#age: 11,
                r#north: true,
            });
        }
        if state_id == 3176 {
            return Some(Fire {
                r#south: true,
                r#up: false,
                r#west: true,
                r#age: 0,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 3575 {
            return Some(Fire {
                r#west: false,
                r#age: 12,
                r#north: true,
                r#up: true,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 3480 {
            return Some(Fire {
                r#south: true,
                r#north: true,
                r#age: 9,
                r#up: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 3661 {
            return Some(Fire {
                r#south: false,
                r#west: false,
                r#age: 15,
                r#north: true,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 3387 {
            return Some(Fire {
                r#south: false,
                r#east: false,
                r#up: true,
                r#age: 6,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 3511 {
            return Some(Fire {
                r#age: 10,
                r#south: true,
                r#up: true,
                r#north: true,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 3652 {
            return Some(Fire {
                r#east: false,
                r#south: false,
                r#age: 14,
                r#up: false,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 3438 {
            return Some(Fire {
                r#up: true,
                r#age: 8,
                r#south: true,
                r#west: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 3642 {
            return Some(Fire {
                r#west: true,
                r#north: true,
                r#south: false,
                r#east: false,
                r#age: 14,
                r#up: true,
            });
        }
        if state_id == 3449 {
            return Some(Fire {
                r#age: 8,
                r#east: false,
                r#up: false,
                r#west: false,
                r#south: true,
                r#north: true,
            });
        }
        if state_id == 3474 {
            return Some(Fire {
                r#east: true,
                r#south: false,
                r#west: true,
                r#north: false,
                r#up: true,
                r#age: 9,
            });
        }
        if state_id == 3413 {
            return Some(Fire {
                r#west: false,
                r#south: false,
                r#up: false,
                r#age: 7,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 3286 {
            return Some(Fire {
                r#up: true,
                r#east: false,
                r#north: true,
                r#south: true,
                r#west: true,
                r#age: 3,
            });
        }
        if state_id == 3428 {
            return Some(Fire {
                r#west: true,
                r#east: false,
                r#age: 7,
                r#south: false,
                r#north: false,
                r#up: false,
            });
        }
        if state_id == 3299 {
            return Some(Fire {
                r#north: false,
                r#age: 3,
                r#south: false,
                r#east: false,
                r#west: false,
                r#up: true,
            });
        }
        if state_id == 3599 {
            return Some(Fire {
                r#south: true,
                r#east: true,
                r#up: true,
                r#west: false,
                r#age: 13,
                r#north: false,
            });
        }
        if state_id == 3624 {
            return Some(Fire {
                r#age: 14,
                r#south: true,
                r#east: true,
                r#up: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 3421 {
            return Some(Fire {
                r#west: false,
                r#east: false,
                r#north: true,
                r#age: 7,
                r#south: false,
                r#up: false,
            });
        }
        if state_id == 3345 {
            return Some(Fire {
                r#south: true,
                r#up: false,
                r#east: true,
                r#west: false,
                r#age: 5,
                r#north: false,
            });
        }
        if state_id == 3350 {
            return Some(Fire {
                r#up: true,
                r#east: false,
                r#south: true,
                r#age: 5,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 3679 {
            return Some(Fire {
                r#age: 15,
                r#east: false,
                r#south: true,
                r#up: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 3288 {
            return Some(Fire {
                r#east: false,
                r#south: true,
                r#west: true,
                r#up: false,
                r#age: 3,
                r#north: true,
            });
        }
        if state_id == 3685 {
            return Some(Fire {
                r#east: false,
                r#up: false,
                r#north: false,
                r#age: 15,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 3627 {
            return Some(Fire {
                r#up: true,
                r#north: true,
                r#east: true,
                r#age: 14,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 3322 {
            return Some(Fire {
                r#north: true,
                r#up: true,
                r#east: false,
                r#age: 4,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 3450 {
            return Some(Fire {
                r#north: true,
                r#east: false,
                r#west: true,
                r#south: false,
                r#up: true,
                r#age: 8,
            });
        }
        if state_id == 3614 {
            return Some(Fire {
                r#up: true,
                r#age: 13,
                r#north: false,
                r#east: false,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 3304 {
            return Some(Fire {
                r#south: true,
                r#age: 4,
                r#up: false,
                r#west: true,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 3498 {
            return Some(Fire {
                r#east: true,
                r#south: false,
                r#west: true,
                r#north: true,
                r#age: 10,
                r#up: true,
            });
        }
        if state_id == 3203 {
            return Some(Fire {
                r#age: 0,
                r#west: false,
                r#south: false,
                r#north: false,
                r#up: true,
                r#east: false,
            });
        }
        if state_id == 3239 {
            return Some(Fire {
                r#age: 2,
                r#north: true,
                r#east: true,
                r#south: true,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3248 {
            return Some(Fire {
                r#south: true,
                r#north: false,
                r#up: false,
                r#west: true,
                r#east: true,
                r#age: 2,
            });
        }
        if state_id == 3669 {
            return Some(Fire {
                r#west: false,
                r#up: false,
                r#east: true,
                r#north: false,
                r#south: false,
                r#age: 15,
            });
        }
        if state_id == 3422 {
            return Some(Fire {
                r#west: true,
                r#north: false,
                r#age: 7,
                r#south: true,
                r#up: true,
                r#east: false,
            });
        }
        if state_id == 3459 {
            return Some(Fire {
                r#age: 8,
                r#east: false,
                r#north: false,
                r#south: false,
                r#west: false,
                r#up: true,
            });
        }
        if state_id == 3303 {
            return Some(Fire {
                r#south: true,
                r#age: 4,
                r#west: false,
                r#east: true,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 3250 {
            return Some(Fire {
                r#north: false,
                r#west: true,
                r#up: true,
                r#south: false,
                r#east: true,
                r#age: 2,
            });
        }
        if state_id == 3674 {
            return Some(Fire {
                r#north: true,
                r#age: 15,
                r#south: false,
                r#east: false,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 3192 {
            return Some(Fire {
                r#south: true,
                r#up: false,
                r#east: false,
                r#west: true,
                r#north: true,
                r#age: 0,
            });
        }
        if state_id == 3468 {
            return Some(Fire {
                r#north: true,
                r#south: false,
                r#east: true,
                r#up: false,
                r#west: true,
                r#age: 9,
            });
        }
        if state_id == 3608 {
            return Some(Fire {
                r#west: true,
                r#up: false,
                r#north: true,
                r#south: true,
                r#east: false,
                r#age: 13,
            });
        }
        if state_id == 3461 {
            return Some(Fire {
                r#north: false,
                r#south: false,
                r#up: false,
                r#age: 8,
                r#east: false,
                r#west: false,
            });
        }
        if state_id == 3536 {
            return Some(Fire {
                r#east: true,
                r#west: true,
                r#south: true,
                r#up: false,
                r#north: false,
                r#age: 11,
            });
        }
        if state_id == 3199 {
            return Some(Fire {
                r#east: false,
                r#up: true,
                r#south: true,
                r#west: false,
                r#age: 0,
                r#north: false,
            });
        }
        if state_id == 3194 {
            return Some(Fire {
                r#age: 0,
                r#east: false,
                r#north: true,
                r#west: true,
                r#up: true,
                r#south: false,
            });
        }
        if state_id == 3351 {
            return Some(Fire {
                r#south: true,
                r#north: true,
                r#up: true,
                r#west: false,
                r#age: 5,
                r#east: false,
            });
        }
        if state_id == 3375 {
            return Some(Fire {
                r#east: true,
                r#age: 6,
                r#south: true,
                r#north: false,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3381 {
            return Some(Fire {
                r#east: true,
                r#north: false,
                r#south: false,
                r#age: 6,
                r#west: false,
                r#up: false,
            });
        }
        if state_id == 3295 {
            return Some(Fire {
                r#north: false,
                r#east: false,
                r#south: true,
                r#up: true,
                r#west: false,
                r#age: 3,
            });
        }
        if state_id == 3410 {
            return Some(Fire {
                r#age: 7,
                r#up: true,
                r#east: true,
                r#north: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 3435 {
            return Some(Fire {
                r#age: 8,
                r#east: true,
                r#north: true,
                r#south: false,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3191 {
            return Some(Fire {
                r#age: 0,
                r#north: true,
                r#south: true,
                r#up: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 3331 {
            return Some(Fire {
                r#up: true,
                r#east: false,
                r#north: false,
                r#west: false,
                r#age: 4,
                r#south: false,
            });
        }
        if state_id == 3496 {
            return Some(Fire {
                r#north: true,
                r#east: true,
                r#west: true,
                r#south: true,
                r#age: 10,
                r#up: false,
            });
        }
        if state_id == 3408 {
            return Some(Fire {
                r#north: false,
                r#west: true,
                r#east: true,
                r#south: true,
                r#up: false,
                r#age: 7,
            });
        }
        if state_id == 3530 {
            return Some(Fire {
                r#east: true,
                r#north: true,
                r#west: true,
                r#south: false,
                r#up: true,
                r#age: 11,
            });
        }
        if state_id == 3533 {
            return Some(Fire {
                r#south: false,
                r#west: false,
                r#north: true,
                r#up: false,
                r#age: 11,
                r#east: true,
            });
        }
        if state_id == 3341 {
            return Some(Fire {
                r#east: true,
                r#west: false,
                r#south: false,
                r#up: false,
                r#age: 5,
                r#north: true,
            });
        }
        if state_id == 3647 {
            return Some(Fire {
                r#west: false,
                r#up: true,
                r#age: 14,
                r#east: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 3399 {
            return Some(Fire {
                r#south: true,
                r#east: true,
                r#north: true,
                r#up: true,
                r#west: false,
                r#age: 7,
            });
        }
        if state_id == 3361 {
            return Some(Fire {
                r#south: true,
                r#north: false,
                r#west: false,
                r#east: false,
                r#up: false,
                r#age: 5,
            });
        }
        if state_id == 3654 {
            return Some(Fire {
                r#north: true,
                r#east: true,
                r#south: true,
                r#up: true,
                r#west: true,
                r#age: 15,
            });
        }
        if state_id == 3240 {
            return Some(Fire {
                r#east: true,
                r#north: true,
                r#age: 2,
                r#south: true,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 3251 {
            return Some(Fire {
                r#east: true,
                r#age: 2,
                r#south: false,
                r#up: true,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 3257 {
            return Some(Fire {
                r#west: false,
                r#south: true,
                r#up: false,
                r#north: true,
                r#age: 2,
                r#east: false,
            });
        }
        if state_id == 3336 {
            return Some(Fire {
                r#age: 5,
                r#east: true,
                r#west: true,
                r#south: true,
                r#north: true,
                r#up: false,
            });
        }
        if state_id == 3476 {
            return Some(Fire {
                r#east: true,
                r#north: false,
                r#up: false,
                r#west: true,
                r#south: false,
                r#age: 9,
            });
        }
        if state_id == 3494 {
            return Some(Fire {
                r#west: true,
                r#age: 10,
                r#north: true,
                r#east: true,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 3658 {
            return Some(Fire {
                r#up: true,
                r#north: true,
                r#west: true,
                r#east: true,
                r#age: 15,
                r#south: false,
            });
        }
        if state_id == 3563 {
            return Some(Fire {
                r#south: false,
                r#east: true,
                r#up: true,
                r#age: 12,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 3547 {
            return Some(Fire {
                r#south: false,
                r#west: false,
                r#east: false,
                r#north: true,
                r#age: 11,
                r#up: true,
            });
        }
        if state_id == 3580 {
            return Some(Fire {
                r#age: 12,
                r#east: false,
                r#up: false,
                r#north: true,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 3589 {
            return Some(Fire {
                r#east: false,
                r#up: false,
                r#age: 12,
                r#north: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 3238 {
            return Some(Fire {
                r#up: true,
                r#west: true,
                r#age: 2,
                r#north: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 3374 {
            return Some(Fire {
                r#north: false,
                r#up: true,
                r#south: true,
                r#age: 6,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 3566 {
            return Some(Fire {
                r#age: 12,
                r#up: true,
                r#north: false,
                r#west: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 3616 {
            return Some(Fire {
                r#east: false,
                r#up: false,
                r#age: 13,
                r#north: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 3478 {
            return Some(Fire {
                r#age: 9,
                r#east: false,
                r#north: true,
                r#west: true,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 3247 {
            return Some(Fire {
                r#age: 2,
                r#up: true,
                r#east: true,
                r#north: false,
                r#west: false,
                r#south: true,
            });
        }
        if state_id == 3676 {
            return Some(Fire {
                r#up: false,
                r#east: false,
                r#west: true,
                r#north: true,
                r#age: 15,
                r#south: false,
            });
        }
        if state_id == 3231 {
            return Some(Fire {
                r#south: true,
                r#north: false,
                r#age: 1,
                r#east: false,
                r#west: false,
                r#up: true,
            });
        }
        if state_id == 3368 {
            return Some(Fire {
                r#up: false,
                r#east: true,
                r#south: true,
                r#north: true,
                r#age: 6,
                r#west: true,
            });
        }
        if state_id == 3525 {
            return Some(Fire {
                r#up: false,
                r#east: false,
                r#south: false,
                r#age: 10,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 3423 {
            return Some(Fire {
                r#north: false,
                r#age: 7,
                r#up: true,
                r#west: false,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 3339 {
            return Some(Fire {
                r#up: true,
                r#east: true,
                r#north: true,
                r#west: false,
                r#south: false,
                r#age: 5,
            });
        }
        if state_id == 3308 {
            return Some(Fire {
                r#west: true,
                r#east: true,
                r#north: true,
                r#age: 4,
                r#south: false,
                r#up: false,
            });
        }
        if state_id == 3488 {
            return Some(Fire {
                r#west: true,
                r#north: false,
                r#up: false,
                r#age: 9,
                r#east: false,
                r#south: true,
            });
        }
        if state_id == 3592 {
            return Some(Fire {
                r#east: true,
                r#west: true,
                r#north: true,
                r#age: 13,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 3188 {
            return Some(Fire {
                r#age: 0,
                r#east: true,
                r#south: false,
                r#north: false,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 3313 {
            return Some(Fire {
                r#age: 4,
                r#east: true,
                r#north: false,
                r#south: true,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 3452 {
            return Some(Fire {
                r#up: false,
                r#west: true,
                r#east: false,
                r#age: 8,
                r#north: true,
                r#south: false,
            });
        }
        if state_id == 3462 {
            return Some(Fire {
                r#age: 9,
                r#north: true,
                r#south: true,
                r#east: true,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 3348 {
            return Some(Fire {
                r#age: 5,
                r#up: false,
                r#west: true,
                r#south: false,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 3444 {
            return Some(Fire {
                r#south: false,
                r#north: false,
                r#east: true,
                r#up: false,
                r#age: 8,
                r#west: true,
            });
        }
        if state_id == 3252 {
            return Some(Fire {
                r#age: 2,
                r#west: true,
                r#south: false,
                r#up: false,
                r#north: false,
                r#east: true,
            });
        }
        if state_id == 3324 {
            return Some(Fire {
                r#age: 4,
                r#up: false,
                r#south: false,
                r#east: false,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 3219 {
            return Some(Fire {
                r#east: true,
                r#north: false,
                r#age: 1,
                r#south: false,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3201 {
            return Some(Fire {
                r#age: 0,
                r#up: false,
                r#east: false,
                r#west: false,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 3446 {
            return Some(Fire {
                r#west: true,
                r#age: 8,
                r#south: true,
                r#east: false,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 3260 {
            return Some(Fire {
                r#north: true,
                r#south: false,
                r#east: false,
                r#up: false,
                r#west: true,
                r#age: 2,
            });
        }
        if state_id == 3649 {
            return Some(Fire {
                r#south: true,
                r#up: false,
                r#east: false,
                r#north: false,
                r#age: 14,
                r#west: false,
            });
        }
        if state_id == 3510 {
            return Some(Fire {
                r#east: false,
                r#north: true,
                r#south: true,
                r#up: true,
                r#west: true,
                r#age: 10,
            });
        }
        if state_id == 3529 {
            return Some(Fire {
                r#south: true,
                r#east: true,
                r#north: true,
                r#west: false,
                r#age: 11,
                r#up: false,
            });
        }
        if state_id == 3620 {
            return Some(Fire {
                r#west: true,
                r#south: false,
                r#age: 13,
                r#north: false,
                r#east: false,
                r#up: false,
            });
        }
        if state_id == 3584 {
            return Some(Fire {
                r#east: false,
                r#north: false,
                r#south: true,
                r#up: false,
                r#west: true,
                r#age: 12,
            });
        }
        if state_id == 3565 {
            return Some(Fire {
                r#north: true,
                r#south: false,
                r#west: false,
                r#up: false,
                r#age: 12,
                r#east: true,
            });
        }
        if state_id == 3390 {
            return Some(Fire {
                r#age: 6,
                r#east: false,
                r#south: true,
                r#north: false,
                r#west: true,
                r#up: true,
            });
        }
        if state_id == 3546 {
            return Some(Fire {
                r#east: false,
                r#south: false,
                r#up: true,
                r#west: true,
                r#age: 11,
                r#north: true,
            });
        }
        if state_id == 3581 {
            return Some(Fire {
                r#age: 12,
                r#east: false,
                r#south: false,
                r#west: false,
                r#up: false,
                r#north: true,
            });
        }
        if state_id == 3562 {
            return Some(Fire {
                r#east: true,
                r#south: false,
                r#up: true,
                r#north: true,
                r#age: 12,
                r#west: true,
            });
        }
        if state_id == 3189 {
            return Some(Fire {
                r#west: false,
                r#north: false,
                r#east: true,
                r#up: false,
                r#age: 0,
                r#south: false,
            });
        }
        if state_id == 3264 {
            return Some(Fire {
                r#south: true,
                r#west: true,
                r#north: false,
                r#east: false,
                r#age: 2,
                r#up: false,
            });
        }
        if state_id == 3419 {
            return Some(Fire {
                r#age: 7,
                r#north: true,
                r#west: false,
                r#east: false,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 3272 {
            return Some(Fire {
                r#west: true,
                r#age: 3,
                r#up: false,
                r#south: true,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 3343 {
            return Some(Fire {
                r#north: false,
                r#age: 5,
                r#south: true,
                r#west: false,
                r#up: true,
                r#east: true,
            });
        }
        if state_id == 3312 {
            return Some(Fire {
                r#south: true,
                r#age: 4,
                r#up: false,
                r#west: true,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 3366 {
            return Some(Fire {
                r#south: true,
                r#east: true,
                r#up: true,
                r#age: 6,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 3373 {
            return Some(Fire {
                r#up: false,
                r#south: false,
                r#west: false,
                r#age: 6,
                r#east: true,
                r#north: true,
            });
        }
        if state_id == 3501 {
            return Some(Fire {
                r#south: false,
                r#west: false,
                r#up: false,
                r#age: 10,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 3504 {
            return Some(Fire {
                r#west: true,
                r#east: true,
                r#south: true,
                r#age: 10,
                r#up: false,
                r#north: false,
            });
        }
        if state_id == 3266 {
            return Some(Fire {
                r#north: false,
                r#up: true,
                r#west: true,
                r#south: false,
                r#age: 2,
                r#east: false,
            });
        }
        if state_id == 3319 {
            return Some(Fire {
                r#east: false,
                r#north: true,
                r#up: true,
                r#west: false,
                r#age: 4,
                r#south: true,
            });
        }
        if state_id == 3400 {
            return Some(Fire {
                r#north: true,
                r#west: true,
                r#age: 7,
                r#east: true,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 3451 {
            return Some(Fire {
                r#north: true,
                r#age: 8,
                r#east: false,
                r#south: false,
                r#west: false,
                r#up: true,
            });
        }
        if state_id == 3505 {
            return Some(Fire {
                r#north: false,
                r#age: 10,
                r#west: false,
                r#east: true,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 3523 {
            return Some(Fire {
                r#south: false,
                r#up: true,
                r#west: false,
                r#east: false,
                r#age: 10,
                r#north: false,
            });
        }
        if state_id == 3282 {
            return Some(Fire {
                r#age: 3,
                r#south: false,
                r#east: true,
                r#west: true,
                r#up: true,
                r#north: false,
            });
        }
        if state_id == 3455 {
            return Some(Fire {
                r#age: 8,
                r#south: true,
                r#north: false,
                r#east: false,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3357 {
            return Some(Fire {
                r#up: false,
                r#north: true,
                r#west: false,
                r#east: false,
                r#south: false,
                r#age: 5,
            });
        }
        if state_id == 3263 {
            return Some(Fire {
                r#west: false,
                r#age: 2,
                r#east: false,
                r#south: true,
                r#up: true,
                r#north: false,
            });
        }
        if state_id == 3302 {
            return Some(Fire {
                r#age: 4,
                r#west: true,
                r#north: true,
                r#east: true,
                r#up: true,
                r#south: true,
            });
        }
        if state_id == 3532 {
            return Some(Fire {
                r#north: true,
                r#up: false,
                r#east: true,
                r#west: true,
                r#age: 11,
                r#south: false,
            });
        }
        if state_id == 3590 {
            return Some(Fire {
                r#west: true,
                r#north: true,
                r#south: true,
                r#up: true,
                r#east: true,
                r#age: 13,
            });
        }
        if state_id == 3195 {
            return Some(Fire {
                r#south: false,
                r#age: 0,
                r#up: true,
                r#east: false,
                r#north: true,
                r#west: false,
            });
        }
        if state_id == 3448 {
            return Some(Fire {
                r#south: true,
                r#east: false,
                r#up: false,
                r#age: 8,
                r#north: true,
                r#west: true,
            });
        }
        if state_id == 3204 {
            return Some(Fire {
                r#north: false,
                r#east: false,
                r#south: false,
                r#up: false,
                r#age: 0,
                r#west: true,
            });
        }
        if state_id == 3433 {
            return Some(Fire {
                r#south: true,
                r#west: false,
                r#east: true,
                r#up: false,
                r#north: true,
                r#age: 8,
            });
        }
        if state_id == 3643 {
            return Some(Fire {
                r#south: false,
                r#age: 14,
                r#north: true,
                r#east: false,
                r#west: false,
                r#up: true,
            });
        }
        if state_id == 3522 {
            return Some(Fire {
                r#up: true,
                r#east: false,
                r#north: false,
                r#age: 10,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 3666 {
            return Some(Fire {
                r#up: true,
                r#age: 15,
                r#west: true,
                r#east: true,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 3321 {
            return Some(Fire {
                r#age: 4,
                r#north: true,
                r#up: false,
                r#south: true,
                r#west: false,
                r#east: false,
            });
        }
        if state_id == 3417 {
            return Some(Fire {
                r#east: false,
                r#south: true,
                r#up: false,
                r#age: 7,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 3495 {
            return Some(Fire {
                r#up: true,
                r#north: true,
                r#age: 10,
                r#east: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 3556 {
            return Some(Fire {
                r#up: false,
                r#east: false,
                r#north: false,
                r#west: true,
                r#age: 11,
                r#south: false,
            });
        }
        if state_id == 3605 {
            return Some(Fire {
                r#west: false,
                r#up: false,
                r#east: true,
                r#north: false,
                r#south: false,
                r#age: 13,
            });
        }
        if state_id == 3265 {
            return Some(Fire {
                r#west: false,
                r#age: 2,
                r#south: true,
                r#east: false,
                r#north: false,
                r#up: false,
            });
        }
        if state_id == 3384 {
            return Some(Fire {
                r#north: true,
                r#east: false,
                r#up: false,
                r#south: true,
                r#age: 6,
                r#west: true,
            });
        }
        if state_id == 3545 {
            return Some(Fire {
                r#south: true,
                r#east: false,
                r#up: false,
                r#west: false,
                r#age: 11,
                r#north: true,
            });
        }
        if state_id == 3280 {
            return Some(Fire {
                r#age: 3,
                r#up: false,
                r#east: true,
                r#west: true,
                r#north: false,
                r#south: true,
            });
        }
        if state_id == 3301 {
            return Some(Fire {
                r#west: false,
                r#up: false,
                r#age: 3,
                r#east: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 3471 {
            return Some(Fire {
                r#east: true,
                r#north: false,
                r#south: true,
                r#age: 9,
                r#west: false,
                r#up: true,
            });
        }
        if state_id == 3262 {
            return Some(Fire {
                r#age: 2,
                r#north: false,
                r#up: true,
                r#west: true,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 3297 {
            return Some(Fire {
                r#west: false,
                r#north: false,
                r#south: true,
                r#age: 3,
                r#east: false,
                r#up: false,
            });
        }
        if state_id == 3392 {
            return Some(Fire {
                r#north: false,
                r#south: true,
                r#up: false,
                r#age: 6,
                r#west: true,
                r#east: false,
            });
        }
        if state_id == 3454 {
            return Some(Fire {
                r#age: 8,
                r#east: false,
                r#north: false,
                r#west: true,
                r#up: true,
                r#south: true,
            });
        }
        if state_id == 3287 {
            return Some(Fire {
                r#west: false,
                r#south: true,
                r#up: true,
                r#age: 3,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 3406 {
            return Some(Fire {
                r#east: true,
                r#north: false,
                r#up: true,
                r#age: 7,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 3472 {
            return Some(Fire {
                r#east: true,
                r#north: false,
                r#west: true,
                r#age: 9,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 3609 {
            return Some(Fire {
                r#east: false,
                r#west: false,
                r#south: true,
                r#north: true,
                r#up: false,
                r#age: 13,
            });
        }
        if state_id == 3570 {
            return Some(Fire {
                r#south: false,
                r#west: true,
                r#north: false,
                r#east: true,
                r#up: true,
                r#age: 12,
            });
        }
        if state_id == 3226 {
            return Some(Fire {
                r#east: false,
                r#south: false,
                r#west: true,
                r#north: true,
                r#age: 1,
                r#up: true,
            });
        }
        if state_id == 3338 {
            return Some(Fire {
                r#south: false,
                r#west: true,
                r#east: true,
                r#north: true,
                r#age: 5,
                r#up: true,
            });
        }
        if state_id == 3551 {
            return Some(Fire {
                r#west: false,
                r#north: false,
                r#south: true,
                r#age: 11,
                r#east: false,
                r#up: true,
            });
        }
        if state_id == 3270 {
            return Some(Fire {
                r#north: true,
                r#south: true,
                r#west: true,
                r#east: true,
                r#up: true,
                r#age: 3,
            });
        }
        if state_id == 3586 {
            return Some(Fire {
                r#north: false,
                r#south: false,
                r#up: true,
                r#east: false,
                r#west: true,
                r#age: 12,
            });
        }
        if state_id == 3541 {
            return Some(Fire {
                r#east: true,
                r#west: false,
                r#north: false,
                r#age: 11,
                r#south: false,
                r#up: false,
            });
        }
        if state_id == 3224 {
            return Some(Fire {
                r#north: true,
                r#west: true,
                r#up: false,
                r#east: false,
                r#south: true,
                r#age: 1,
            });
        }
        if state_id == 3254 {
            return Some(Fire {
                r#up: true,
                r#north: true,
                r#east: false,
                r#age: 2,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 3431 {
            return Some(Fire {
                r#south: true,
                r#east: true,
                r#north: true,
                r#up: true,
                r#age: 8,
                r#west: false,
            });
        }
        if state_id == 3576 {
            return Some(Fire {
                r#age: 12,
                r#north: true,
                r#south: true,
                r#east: false,
                r#west: true,
                r#up: false,
            });
        }
        if state_id == 3207 {
            return Some(Fire {
                r#up: true,
                r#west: false,
                r#age: 1,
                r#east: true,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 3615 {
            return Some(Fire {
                r#west: false,
                r#up: true,
                r#east: false,
                r#north: false,
                r#south: true,
                r#age: 13,
            });
        }
        if state_id == 3485 {
            return Some(Fire {
                r#south: false,
                r#north: true,
                r#east: false,
                r#up: false,
                r#west: false,
                r#age: 9,
            });
        }
        if state_id == 3623 {
            return Some(Fire {
                r#north: true,
                r#age: 14,
                r#east: true,
                r#up: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 3524 {
            return Some(Fire {
                r#south: false,
                r#up: false,
                r#east: false,
                r#age: 10,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 3519 {
            return Some(Fire {
                r#up: true,
                r#west: false,
                r#age: 10,
                r#south: true,
                r#north: false,
                r#east: false,
            });
        }
        if state_id == 3564 {
            return Some(Fire {
                r#south: false,
                r#west: true,
                r#age: 12,
                r#up: false,
                r#north: true,
                r#east: true,
            });
        }
        if state_id == 3398 {
            return Some(Fire {
                r#age: 7,
                r#east: true,
                r#up: true,
                r#north: true,
                r#west: true,
                r#south: true,
            });
        }
        if state_id == 3340 {
            return Some(Fire {
                r#age: 5,
                r#south: false,
                r#east: true,
                r#west: true,
                r#up: false,
                r#north: true,
            });
        }
        if state_id == 3621 {
            return Some(Fire {
                r#age: 13,
                r#south: false,
                r#up: false,
                r#east: false,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 3187 {
            return Some(Fire {
                r#up: true,
                r#west: false,
                r#age: 0,
                r#north: false,
                r#south: false,
                r#east: true,
            });
        }
        if state_id == 3656 {
            return Some(Fire {
                r#west: true,
                r#south: true,
                r#east: true,
                r#north: true,
                r#up: false,
                r#age: 15,
            });
        }
        if state_id == 3296 {
            return Some(Fire {
                r#up: false,
                r#age: 3,
                r#east: false,
                r#north: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 3405 {
            return Some(Fire {
                r#up: false,
                r#age: 7,
                r#north: true,
                r#east: true,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 3205 {
            return Some(Fire {
                r#north: false,
                r#south: false,
                r#east: false,
                r#age: 0,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 3246 {
            return Some(Fire {
                r#up: true,
                r#east: true,
                r#age: 2,
                r#north: false,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 3662 {
            return Some(Fire {
                r#up: true,
                r#west: true,
                r#south: true,
                r#age: 15,
                r#east: true,
                r#north: false,
            });
        }
        if state_id == 3215 {
            return Some(Fire {
                r#east: true,
                r#up: true,
                r#west: false,
                r#north: false,
                r#age: 1,
                r#south: true,
            });
        }
        if state_id == 3639 {
            return Some(Fire {
                r#south: true,
                r#north: true,
                r#east: false,
                r#up: true,
                r#age: 14,
                r#west: false,
            });
        }
        if state_id == 3434 {
            return Some(Fire {
                r#north: true,
                r#up: true,
                r#east: true,
                r#age: 8,
                r#south: false,
                r#west: true,
            });
        }
        if state_id == 3644 {
            return Some(Fire {
                r#age: 14,
                r#east: false,
                r#south: false,
                r#up: false,
                r#west: true,
                r#north: true,
            });
        }
        if state_id == 3677 {
            return Some(Fire {
                r#west: false,
                r#east: false,
                r#age: 15,
                r#up: false,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 3432 {
            return Some(Fire {
                r#south: true,
                r#east: true,
                r#up: false,
                r#north: true,
                r#age: 8,
                r#west: true,
            });
        }
        if state_id == 3355 {
            return Some(Fire {
                r#south: false,
                r#west: false,
                r#up: true,
                r#east: false,
                r#age: 5,
                r#north: true,
            });
        }
        if state_id == 3271 {
            return Some(Fire {
                r#up: true,
                r#south: true,
                r#north: true,
                r#west: false,
                r#east: true,
                r#age: 3,
            });
        }
        if state_id == 3420 {
            return Some(Fire {
                r#up: false,
                r#south: false,
                r#west: true,
                r#age: 7,
                r#east: false,
                r#north: true,
            });
        }
        if state_id == 3427 {
            return Some(Fire {
                r#east: false,
                r#age: 7,
                r#south: false,
                r#west: false,
                r#up: true,
                r#north: false,
            });
        }
        if state_id == 3397 {
            return Some(Fire {
                r#age: 6,
                r#north: false,
                r#east: false,
                r#south: false,
                r#west: false,
                r#up: false,
            });
        }
        if state_id == 3460 {
            return Some(Fire {
                r#east: false,
                r#up: false,
                r#age: 8,
                r#north: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 3278 {
            return Some(Fire {
                r#east: true,
                r#west: true,
                r#south: true,
                r#north: false,
                r#age: 3,
                r#up: true,
            });
        }
        if state_id == 3499 {
            return Some(Fire {
                r#west: false,
                r#east: true,
                r#south: false,
                r#age: 10,
                r#up: true,
                r#north: true,
            });
        }
        if state_id == 3198 {
            return Some(Fire {
                r#west: true,
                r#north: false,
                r#east: false,
                r#age: 0,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 3365 {
            return Some(Fire {
                r#up: false,
                r#north: false,
                r#east: false,
                r#age: 5,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 3228 {
            return Some(Fire {
                r#east: false,
                r#west: true,
                r#north: true,
                r#age: 1,
                r#up: false,
                r#south: false,
            });
        }
        if state_id == 3416 {
            return Some(Fire {
                r#south: true,
                r#north: true,
                r#age: 7,
                r#up: false,
                r#east: false,
                r#west: true,
            });
        }
        if state_id == 3625 {
            return Some(Fire {
                r#west: false,
                r#north: true,
                r#east: true,
                r#south: true,
                r#age: 14,
                r#up: false,
            });
        }
        if state_id == 3414 {
            return Some(Fire {
                r#east: false,
                r#age: 7,
                r#north: true,
                r#up: true,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 3537 {
            return Some(Fire {
                r#north: false,
                r#east: true,
                r#south: true,
                r#age: 11,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 3273 {
            return Some(Fire {
                r#up: false,
                r#north: true,
                r#south: true,
                r#east: true,
                r#west: false,
                r#age: 3,
            });
        }
        if state_id == 3542 {
            return Some(Fire {
                r#age: 11,
                r#north: true,
                r#east: false,
                r#south: true,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 3597 {
            return Some(Fire {
                r#east: true,
                r#north: true,
                r#south: false,
                r#up: false,
                r#west: false,
                r#age: 13,
            });
        }
        if state_id == 3463 {
            return Some(Fire {
                r#north: true,
                r#south: true,
                r#west: false,
                r#up: true,
                r#age: 9,
                r#east: true,
            });
        }
        if state_id == 3401 {
            return Some(Fire {
                r#south: true,
                r#north: true,
                r#age: 7,
                r#west: false,
                r#east: true,
                r#up: false,
            });
        }
        if state_id == 3668 {
            return Some(Fire {
                r#east: true,
                r#south: false,
                r#up: false,
                r#west: true,
                r#age: 15,
                r#north: false,
            });
        }
        if state_id == 3477 {
            return Some(Fire {
                r#west: false,
                r#east: true,
                r#north: false,
                r#age: 9,
                r#south: false,
                r#up: false,
            });
        }
        if state_id == 3507 {
            return Some(Fire {
                r#east: true,
                r#age: 10,
                r#west: false,
                r#north: false,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 3606 {
            return Some(Fire {
                r#north: true,
                r#up: true,
                r#east: false,
                r#age: 13,
                r#south: true,
                r#west: true,
            });
        }
        if state_id == 3208 {
            return Some(Fire {
                r#south: true,
                r#up: false,
                r#west: true,
                r#east: true,
                r#age: 1,
                r#north: true,
            });
        }
        if state_id == 3378 {
            return Some(Fire {
                r#age: 6,
                r#east: true,
                r#west: true,
                r#north: false,
                r#up: true,
                r#south: false,
            });
        }
        if state_id == 3212 {
            return Some(Fire {
                r#age: 1,
                r#west: true,
                r#up: false,
                r#east: true,
                r#south: false,
                r#north: true,
            });
        }
        if state_id == 3402 {
            return Some(Fire {
                r#south: false,
                r#age: 7,
                r#north: true,
                r#up: true,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 3567 {
            return Some(Fire {
                r#east: true,
                r#north: false,
                r#age: 12,
                r#west: false,
                r#south: true,
                r#up: true,
            });
        }
        if state_id == 3619 {
            return Some(Fire {
                r#north: false,
                r#east: false,
                r#up: true,
                r#age: 13,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 3659 {
            return Some(Fire {
                r#age: 15,
                r#south: false,
                r#east: true,
                r#north: true,
                r#up: true,
                r#west: false,
            });
        }
        if state_id == 3253 {
            return Some(Fire {
                r#age: 2,
                r#east: true,
                r#north: false,
                r#up: false,
                r#south: false,
                r#west: false,
            });
        }
        if state_id == 3330 {
            return Some(Fire {
                r#up: true,
                r#west: true,
                r#age: 4,
                r#east: false,
                r#north: false,
                r#south: false,
            });
        }
        if state_id == 3569 {
            return Some(Fire {
                r#up: false,
                r#west: false,
                r#east: true,
                r#north: false,
                r#age: 12,
                r#south: true,
            });
        }
        if state_id == 3583 {
            return Some(Fire {
                r#west: false,
                r#east: false,
                r#north: false,
                r#south: true,
                r#up: true,
                r#age: 12,
            });
        }
        if state_id == 3598 {
            return Some(Fire {
                r#up: true,
                r#age: 13,
                r#north: false,
                r#west: true,
                r#east: true,
                r#south: true,
            });
        }
        if state_id == 3213 {
            return Some(Fire {
                r#north: true,
                r#east: true,
                r#south: false,
                r#west: false,
                r#age: 1,
                r#up: false,
            });
        }
        if state_id == 3367 {
            return Some(Fire {
                r#age: 6,
                r#east: true,
                r#up: true,
                r#north: true,
                r#south: true,
                r#west: false,
            });
        }
        if state_id == 3631 {
            return Some(Fire {
                r#north: false,
                r#east: true,
                r#south: true,
                r#age: 14,
                r#west: false,
                r#up: true,
            });
        }
        if state_id == 3245 {
            return Some(Fire {
                r#up: false,
                r#north: true,
                r#age: 2,
                r#west: false,
                r#east: true,
                r#south: false,
            });
        }
        if state_id == 3268 {
            return Some(Fire {
                r#north: false,
                r#south: false,
                r#west: true,
                r#east: false,
                r#up: false,
                r#age: 2,
            });
        }
        if state_id == 3578 {
            return Some(Fire {
                r#age: 12,
                r#north: true,
                r#south: false,
                r#west: true,
                r#east: false,
                r#up: true,
            });
        }
        if state_id == 3209 {
            return Some(Fire {
                r#east: true,
                r#west: false,
                r#south: true,
                r#north: true,
                r#age: 1,
                r#up: false,
            });
        }
        if state_id == 3223 {
            return Some(Fire {
                r#up: true,
                r#west: false,
                r#north: true,
                r#south: true,
                r#age: 1,
                r#east: false,
            });
        }
        if state_id == 3377 {
            return Some(Fire {
                r#east: true,
                r#south: true,
                r#up: false,
                r#west: false,
                r#age: 6,
                r#north: false,
            });
        }
        if state_id == 3481 {
            return Some(Fire {
                r#west: false,
                r#age: 9,
                r#east: false,
                r#north: true,
                r#south: true,
                r#up: false,
            });
        }
        if state_id == 3380 {
            return Some(Fire {
                r#age: 6,
                r#east: true,
                r#north: false,
                r#south: false,
                r#up: false,
                r#west: true,
            });
        }
        if state_id == 3391 {
            return Some(Fire {
                r#south: true,
                r#up: true,
                r#age: 6,
                r#east: false,
                r#north: false,
                r#west: false,
            });
        }
        if state_id == 3436 {
            return Some(Fire {
                r#north: true,
                r#west: true,
                r#south: false,
                r#up: false,
                r#age: 8,
                r#east: true,
            });
        }
        if state_id == 3515 {
            return Some(Fire {
                r#west: false,
                r#east: false,
                r#south: false,
                r#age: 10,
                r#north: true,
                r#up: true,
            });
        }
        if state_id == 3572 {
            return Some(Fire {
                r#east: true,
                r#age: 12,
                r#south: false,
                r#up: false,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 3579 {
            return Some(Fire {
                r#north: true,
                r#age: 12,
                r#west: false,
                r#east: false,
                r#south: false,
                r#up: true,
            });
        }
        if state_id == 3588 {
            return Some(Fire {
                r#north: false,
                r#age: 12,
                r#east: false,
                r#up: false,
                r#west: true,
                r#south: false,
            });
        }
        if state_id == 3612 {
            return Some(Fire {
                r#west: true,
                r#age: 13,
                r#north: true,
                r#south: false,
                r#east: false,
                r#up: false,
            });
        }
        if state_id == 3403 {
            return Some(Fire {
                r#east: true,
                r#west: false,
                r#north: true,
                r#age: 7,
                r#up: true,
                r#south: false,
            });
        }
        if state_id == 3486 {
            return Some(Fire {
                r#north: false,
                r#age: 9,
                r#south: true,
                r#west: true,
                r#east: false,
                r#up: true,
            });
        }
        if state_id == 3457 {
            return Some(Fire {
                r#west: false,
                r#age: 8,
                r#south: true,
                r#north: false,
                r#up: false,
                r#east: false,
            });
        }
        if state_id == 3233 {
            return Some(Fire {
                r#age: 1,
                r#up: false,
                r#west: false,
                r#north: false,
                r#south: true,
                r#east: false,
            });
        }
        if state_id == 3352 {
            return Some(Fire {
                r#north: true,
                r#east: false,
                r#south: true,
                r#age: 5,
                r#west: true,
                r#up: false,
            });
        }
        if state_id == 3329 {
            return Some(Fire {
                r#east: false,
                r#north: false,
                r#age: 4,
                r#south: true,
                r#up: false,
                r#west: false,
            });
        }
        if state_id == 3356 {
            return Some(Fire {
                r#up: false,
                r#east: false,
                r#west: true,
                r#north: true,
                r#age: 5,
                r#south: false,
            });
        }
        if state_id == 3227 {
            return Some(Fire {
                r#age: 1,
                r#up: true,
                r#north: true,
                r#east: false,
                r#west: false,
                r#south: false,
            });
        }
        if state_id == 3294 {
            return Some(Fire {
                r#up: true,
                r#age: 3,
                r#south: true,
                r#east: false,
                r#north: false,
                r#west: true,
            });
        }
        if state_id == 3465 {
            return Some(Fire {
                r#west: false,
                r#age: 9,
                r#east: true,
                r#up: false,
                r#north: true,
                r#south: true,
            });
        }
        if state_id == 3587 {
            return Some(Fire {
                r#south: false,
                r#up: true,
                r#west: false,
                r#age: 12,
                r#east: false,
                r#north: false,
            });
        }
        if state_id == 3678 {
            return Some(Fire {
                r#age: 15,
                r#south: true,
                r#east: false,
                r#north: false,
                r#up: true,
                r#west: true,
            });
        }
        if state_id == 3651 {
            return Some(Fire {
                r#south: false,
                r#age: 14,
                r#east: false,
                r#up: true,
                r#west: false,
                r#north: false,
            });
        }
        if state_id == 3333 {
            return Some(Fire {
                r#north: false,
                r#age: 4,
                r#south: false,
                r#west: false,
                r#east: false,
                r#up: false,
            });
        }
        if state_id == 3521 {
            return Some(Fire {
                r#age: 10,
                r#south: true,
                r#west: false,
                r#east: false,
                r#up: false,
                r#north: false,
            });
        }
        if state_id == 3559 {
            return Some(Fire {
                r#age: 12,
                r#east: true,
                r#south: true,
                r#up: true,
                r#west: false,
                r#north: true,
            });
        }
        if state_id == 3326 {
            return Some(Fire {
                r#east: false,
                r#south: true,
                r#up: true,
                r#age: 4,
                r#west: true,
                r#north: false,
            });
        }
        if state_id == 3440 {
            return Some(Fire {
                r#age: 8,
                r#south: true,
                r#north: false,
                r#up: false,
                r#east: true,
                r#west: true,
            });
        }
        if state_id == 3632 {
            return Some(Fire {
                r#south: true,
                r#east: true,
                r#north: false,
                r#up: false,
                r#west: true,
                r#age: 14,
            });
        }
        if state_id == 3469 {
            return Some(Fire {
                r#north: true,
                r#east: true,
                r#up: false,
                r#west: false,
                r#age: 9,
                r#south: false,
            });
        }
        if state_id == 3177 {
            return Some(Fire {
                r#west: false,
                r#age: 0,
                r#east: true,
                r#north: true,
                r#south: true,
                r#up: false,
            });
        }
        return None;
    }
}

