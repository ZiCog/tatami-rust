#[cfg(feature = "use_u32")]
pub mod defs {
    pub type Int = u32;
    pub type PrimeType = u32;
    pub const PNUM: usize = 1_300;
    pub const SMAX: Int = 100_000_000;
    pub const FNUM: usize = 10;
}

#[cfg(feature = "use_u64")]
pub mod defs {
    pub type Int = u64;
    pub type PrimeType = u64;
    pub const PNUM: usize = 40_000;
    pub const SMAX: Int = 100_000_000_000;
    pub const FNUM: usize = 20;
}

