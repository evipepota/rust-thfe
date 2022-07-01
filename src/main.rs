mod tlwe;
mod trlwe;
mod calc;
use calc::complex;

//type Torus = u32;

fn main() {
    tlwe::test(1);
    tlwe::test(0);
    trlwe::test();
}
