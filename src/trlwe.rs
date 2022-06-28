use crate::calc::{fft, complex::{Complex, self}};

type Torus = u32;
const ALPHA: f64 = 1.0 / 2i64.pow(25) as f64;
pub const N: usize = 1024;

pub fn a() {
    let b = Complex{
        re: 1.0,
        im: 1.0
    };
    let mut c : [Complex; N] = [b; N];
    let mut d : [Complex; N] = [b; N];
    fft::convolution(&mut c, &mut d);
}
