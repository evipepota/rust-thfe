use crate::{complex::Complex, trlwe};

fn ufft(a: &mut [Complex; trlwe::N]) {
    let n = a.len();
    let mut m = n;

    while m > 1 {
        for s in 0..(n / m) {
            for p in 0..(m / 2) {
                let l = a[s * m + p];
                let r = a[s * m + p + m / 2];
                a[s * m + p] = l + r;
                a[s * m + p + m / 2] = (l - r) * Complex::omega(m, p as i64);
            }
        }
        m /= 2;
    }
}

fn iufft(a: &mut [Complex; trlwe::N]) {
    // inv
    let n = a.len();
    let mut m = 2;

    while m <= n {
        for s in 0..(n / m) {
            for p in 0..(m / 2) {
                let l = a[s * m + p];
                let r = a[s * m + p + m / 2] * Complex::omega(m, -(p as i64));
                a[s * m + p] = l + r;
                a[s * m + p + m / 2] = l - r;
            }
        }
        m *= 2;
    }

    for i in a {
        // to do check
        i.re /= n as f64;
        i.im /= n as f64;
    }
}

pub fn convolution(a: &mut [Complex; trlwe::N], b: &mut [Complex; trlwe::N]) {
    // to do [int] -> [complex] transerate
    ufft(a);
    ufft(b);

}
