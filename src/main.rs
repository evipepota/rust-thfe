mod calc;
mod sampleextractindex;
mod tlwe;
mod trgsw;
mod trlwe;
mod param;
use calc::{
    complex,
    fft::{convolution_mod, fft_test},
};
use trgsw::nand_test;

use rand::Rng;

//type Torus = u32;

fn main() {
    //cmuxtest();
    //test_blindrotate();
    nand_test();
    //test_identity_key_swwitch();
    //trlwe::test_trlwe();
    //tlwe::test(1);
    //tlwe::test(0);
    //fft_testtest();
}

pub fn fft_testtest() {
    let mut s: [u32; 512] = [0; 512];
    let mut a: [u32; 512] = [0; 512];
    let mut rng = rand::thread_rng();
    for i in s.iter_mut() {
        *i = rng.gen::<u32>();
    }
    for i in a.iter_mut() {
        *i = rng.gen::<u32>();
    }
    let a_s = convolution_mod(&a, &s);
    let a_stest = fft_test(&a, &s);
    for i in 0..512 {
        if a_s[i] != a_stest[i] {
            println!("{}, {}", a_s[i], a_stest[i]);
        }
    }
}
