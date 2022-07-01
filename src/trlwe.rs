use crate::calc::{fft, complex::{Complex, self}};

type Torus = u32;
const ALPHA: f64 = 1.0 / 2i64.pow(25) as f64;
pub const N: usize = 512;
pub const K: usize = 2;
