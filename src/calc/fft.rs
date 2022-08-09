use num::traits::WrappingSub;

use crate::complex::Complex;
type Torus = u32;

fn ufft(a: &mut [Complex]) {
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

fn iufft(a: &mut [Complex]) {
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

fn fft(a: &[Torus], b: &[Torus]) -> Vec<i128> {
    let n = a.len();
    let mut a_M = vec![0; n];
    let mut b_M = vec![0; n];
    let mut a_mM = vec![0; n];
    let mut b_mM = vec![0; n];
    let mut a_m = vec![0; n];
    let mut b_m = vec![0; n];

    let div = 1 << 16;
    for i in 0..n {
        a_M[i] = a[i] / div;
        a_m[i] = a[i] % div;
        a_mM[i] = a_m[i] + a_M[i];
        b_M[i] = b[i] / div;
        b_m[i] = b[i] % div;
        b_mM[i] = b_m[i] + b_M[i];
    }
    let M = convolution(&a_M, &b_M);
    let mM = convolution(&a_mM, &b_mM);
    let m = convolution(&a_m, &b_m);

    let mut res = vec![0; 2 * n];
    for i in 0..2 * n {
        res[i] = (M[i] << (32)) + ((mM[i] - m[i] - M[i]) << 16) + m[i];
    }
    res
}

pub fn convolution(a: &[Torus], b: &[Torus]) -> Vec<i128> {
    let n = a.len();
    let mut com_a: Vec<Complex> = vec![Complex { re: 0.0, im: 0.0 }; 2 * n];
    let mut com_b: Vec<Complex> = vec![Complex { re: 0.0, im: 0.0 }; 2 * n];
    for i in 0..n {
        com_a[i].re = a[i] as f64;
        com_b[i].re = b[i] as f64;
    }
    ufft(&mut com_a);
    ufft(&mut com_b);
    for i in 0..2 * n {
        com_a[i] *= com_b[i];
    }
    iufft(&mut com_a);

    let mut ans = vec![0; 2 * n];
    for i in 0..2 * n {
        let k = com_a[i].re.round() as i128;
        ans[i] = k;
    }
    return ans;
}

pub fn convolution_mod(a: &[Torus], b: &[Torus]) -> Vec<Torus> {
    let n = a.len();
    //let ab = convolution(a, b);
    let mut k = fft(a, b);
    let mut res: Vec<Torus> = vec![0; n];
    for i in 0..2 * n {
        k[i] %= u64::MAX as i128;
        if k[i] < 0 {
            k[i] += u64::MAX as i128;
        }
        if i < n {
            res[i] = res[i].wrapping_add(k[i] as u32);
        } else {
            res[i - n] = res[i - n].wrapping_sub(k[i] as u32);
        }
    }

    return res;
}

pub fn fft_test(a: &[Torus], b: &[Torus]) -> Vec<Torus> {
    let n = a.len();
    let mut res: Vec<Torus> = vec![0; n];

    for i in 0..n {
        for j in 0..n {
            if i + j < n {
                res[i + j] = res[i + j].wrapping_add(a[i].wrapping_mul(b[j]));
            } else {
                res[i + j - n] = res[i + j - n].wrapping_sub(a[i].wrapping_mul(b[j]));
            }
        }
    }

    res
}

/*pub fn fft_test2(a: &[Torus], b: &[Torus]) -> Vec<i128> {
    let n = a.len();
    let mut res: Vec<i128> = vec![0; 2*n];

    for i in 0..n {
        for j in 0..n {
            res[i+j] += (a[i] as i128)*(b[j] as i128);
        }
    }

    res
}*/
