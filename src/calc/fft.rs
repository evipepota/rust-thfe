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

    /*for i in a {
        // to do check
        i.re /= n as f64;
        i.im /= n as f64;
    }*/

    //n = N/2
    //2N = 4n
    for i in 0..n {
        a[i] /= Complex::omega(4 * n, i as i64);
        a[i].re /= n as f64;
        a[i].im /= n as f64;
    }
}

pub fn convolution(a: &[i32], b: &[i32]) -> Vec<i64> {
    let n = a.len();
    let mut com_a: Vec<Complex> = vec![Complex { re: 0.0, im: 0.0 }; n / 2];
    let mut com_b: Vec<Complex> = vec![Complex { re: 0.0, im: 0.0 }; n / 2];
    for i in 0..n {
        if i < n / 2 {
            com_a[i].re = a[i] as f64;
            com_b[i].re = b[i] as f64;
        } else {
            com_a[i - n / 2].im = a[i] as f64;
            com_a[i - n / 2] *= Complex::omega(2 * n, i as i64 - n as i64 / 2);
            com_b[i - n / 2].im = b[i] as f64;
            com_b[i - n / 2] *= Complex::omega(2 * n, i as i64 - n as i64 / 2);
        }
    }
    ufft(&mut com_a);
    ufft(&mut com_b);
    for i in 0..n / 2 {
        com_a[i] *= com_b[i];
    }
    iufft(&mut com_a);

    let mut ans = vec![0; 2 * n];
    for i in 0..n / 2 {
        let mut k = com_a[i].re.round() as i64;
        k %= u32::MAX as i64;
        if k < 0 {
            k += u32::MAX as i64;
        }
        ans[i] = k;
        let mut k = com_a[i].im.round() as i64;
        k %= u32::MAX as i64;
        if k < 0 {
            k += u32::MAX as i64;
        }
        ans[i + n / 2] = k;
    }
    return ans;
}

pub fn convolution_mod(a: &[Torus], b: &[Torus]) -> Vec<Torus> {
    let n = a.len();
    //let ab = convolution(a, b);

    //torus -> i64
    let mut a_i64 = vec![0; n];
    let mut b_i64 = vec![0; n];
    for i in 0..n {
        /*if a[i] > (u32::MAX/2) {
            a_i64[i] = a[i] as i64 - (u32::MAX as i64);
        } else {
            a_i64[i] = a[i] as i64;
        }
        if b[i] > (u32::MAX/2) {
            b_i64[i] = b[i] as i64 - (u32::MAX as i64);
        } else {
            b_i64[i] = b[i] as i64;
        }*/
        a_i64[i] = a[i] as i32;
        b_i64[i] = b[i] as i32;
    }
    let mut k = convolution(&a_i64, &b_i64);

    let mut res: Vec<Torus> = vec![0; 2 * n];
    for i in 0..n {
        k[i] %= u32::MAX as i64;
        if k[i] < 0 {
            k[i] += u32::MAX as i64;
        }
        if i < n {
            res[i] = res[i].wrapping_add(k[i] as u32);
        } else {
            res[i - n] = res[i - n].wrapping_sub(k[i] as u32);
        }
    }
    res
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
