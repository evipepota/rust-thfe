pub mod tlwe_lvl0 {
    pub const ALPHA: f64 = 0.0000925119974676756;
    pub const N: usize = 586;
}

pub mod trlwe {
    pub const ALPHA: f64 = 0.0000000342338787018369;
    pub const N: usize = 512;
    pub const K: usize = 2;
}

pub mod trgsw {
    pub const N: usize = 512; //equal trlwe::N
    pub const K: usize = 2; //equal trlwe::K
    pub const NBIT: usize = 9;
    pub const L: usize = 2;
    pub const BGBIT: usize = 8;
    pub const BG: u32 = 1 << BGBIT;
    pub const T: usize = 5;
    pub const BASEBIT: usize = 2;
    pub const BASE: usize = 1 << BASEBIT;
}
