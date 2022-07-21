mod tlwe;
mod trlwe;
mod calc;
mod sampleextractindex;
use calc::complex;

//type Torus = u32;

fn main() {
    tlwe::test(1, tlwe::N);
    tlwe::test(0, tlwe::N);
    trlwe::test(trlwe::N, trlwe::K);
}
