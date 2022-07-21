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

pub fn convolution(a: &[Torus], b: &[Torus]) -> Vec<Torus> {
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
    let mut ans: Vec<Torus> = vec![0; 2 * n];
    for i in 0..2 * n {
        ans[i] = com_a[i].re.round() as Torus;
    }
    return ans;
}

pub fn convolution_mod(a: &[Torus], b: &[Torus]) -> Vec<Torus> {
    let n = a.len();
    let ab = convolution(a, b);
    let mut res: Vec<Torus> = vec![0; n];
    for i in 0..2 * n {
        if i < n {
            res[i] = res[i].wrapping_add(ab[i]);
        } else {
            res[i - n] = res[i - n].wrapping_sub(ab[i]);
        }
    }
    return res;
}
