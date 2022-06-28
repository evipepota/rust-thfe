use crate::{complex::Complex, trlwe};
type Torus = u32;

fn ufft(a: &mut [Complex; 2*trlwe::N]) {
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

fn iufft(a: &mut [Complex; 2*trlwe::N]) {
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

pub fn convolution(a: [Torus; trlwe::N], b: [Torus; trlwe::N]) -> [Torus; 2*trlwe::N]{
    let mut com_a: [Complex; 2*trlwe::N] = [Complex{re:0.0, im:0.0}; 2*trlwe::N];
    let mut com_b: [Complex; 2*trlwe::N] = [Complex{re:0.0, im:0.0}; 2*trlwe::N];
    for i in 0..trlwe::N {
        com_a[i].re = a[i] as f64;
        com_b[i].re = b[i] as f64;
    }
    ufft(&mut com_a);
    ufft(&mut com_b);
    for i in 0..2*trlwe::N {
        com_a[i] *= com_b[i];
    }
    iufft(&mut com_a);
    let mut ans: [Torus; 2*trlwe::N] = [0; 2*trlwe::N];
    for i in 0..2*trlwe::N {
        ans[i] = com_a[i].re.round() as Torus;
    }
    return ans;
}
