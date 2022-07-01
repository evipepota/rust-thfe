use crate::{complex::Complex, trlwe};
type Torus = u32;
const N: usize = trlwe::N;

fn ufft(a: &mut [Complex; 2 * N]) {
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

fn iufft(a: &mut [Complex; 2 * N]) {
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

pub fn convolution(a: [Torus; N], b: [Torus; N]) -> [Torus; 2 * N] {
    let mut com_a: [Complex; 2 * N] = [Complex { re: 0.0, im: 0.0 }; 2 * N];
    let mut com_b: [Complex; 2 * N] = [Complex { re: 0.0, im: 0.0 }; 2 * N];
    for i in 0..N {
        com_a[i].re = a[i] as f64;
        com_b[i].re = b[i] as f64;
    }
    ufft(&mut com_a);
    ufft(&mut com_b);
    for i in 0..2 * N {
        com_a[i] *= com_b[i];
    }
    iufft(&mut com_a);
    let mut ans: [Torus; 2 * N] = [0; 2 * N];
    for i in 0..2 * N {
        ans[i] = com_a[i].re.round() as Torus;
    }
    return ans;
}

pub fn convolution_mod(a: [Torus; N], b: [Torus; N]) -> [Torus; N] {
    let ab = convolution(a, b);
    let mut res: [Torus; N] = [0; N];
    for i in 0..2 * N {
        if i < N {
            res[i] = res[i].wrapping_add(ab[i]);
        } else {
            res[i - N] = res[i - N].wrapping_sub(ab[i]);
        }
    }
    return res;
}
