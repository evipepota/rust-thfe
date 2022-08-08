mod tlwe;
mod trlwe;
mod calc;
mod sampleextractindex;
mod trgsw;
use calc::complex;
use trgsw::{nand_test, test_blindrotate, test_identity_key_swwitch};

use crate::calc::fft::convolution;

//type Torus = u32;

fn main() {
    //cmuxtest();
    //test_blindrotate();
    nand_test();
    //test_identity_key_swwitch();
    //trlwe::test();
    //tlwe::test(1, tlwe::N);
    //tlwe::test(0, tlwe::N);
}
