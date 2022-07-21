use rand::Rng;

use crate::calc::{fft::convolution_mod, torus_tool};

type Torus = u32;
const ALPHA: f64 = 1.0 / 2i64.pow(25) as f64;
pub const N: usize = 512;
pub const K: usize = 2;

pub struct TrlweKey {
    pub key_s: Vec<Vec<u32>>,
    pub key_e: Vec<Torus>,
}

pub struct TrlweEncryption {
    pub a: Vec<Vec<Torus>>,
    pub b: Vec<Torus>,
}

impl TrlweKey {
    pub fn keygen(k: usize, n: usize) -> Self {
        //generate s, e
        let mut e: Vec<Torus> = vec![0; n];
        for i in e.iter_mut() {
            *i = torus_tool::d_ta(ALPHA);
        }
        let mut s: Vec<Vec<u32>> = vec![vec![0; n]; k];
        let mut rng = rand::thread_rng();
        for i in s.iter_mut() {
            for j in i.iter_mut() {
                *j = rng.gen_range(0..2);
            }
        }
        Self {
            key_s: (s),
            key_e: (e),
        }
    }
}

pub struct Trlwe {
    pub key: TrlweKey,
    pub enc: TrlweEncryption,
    //pub dec: Vec<i64>,
}

impl Trlwe {
    pub fn new_trlwe(bit: &[i64], n: usize, k: usize) -> Trlwe {
        let key = TrlweKey::keygen(k, n);
        let abs = encrypt(bit, &key, n, k);
        //let ans = decrypt(&abs.a, &abs.b, &key.key_s, n, k);
        Trlwe {
            key: (key),
            enc: abs,
            //dec: (ans),
        }
    }
}

pub fn encrypt(m: &[i64], key: &TrlweKey, n: usize, k: usize) -> TrlweEncryption {
    let mut a: Vec<Vec<Torus>> = vec![vec![0; n]; k];
    let mut m2: Vec<Torus> = vec![0; n];
    for i in 0..n {
        m2[i] = torus_tool::f2torus(((2 * m[i] - 1) as f64) / 8.0);
    }
    let mut b: Vec<Torus> = vec![0; n];
    let s = &key.key_s;
    let mut rng = rand::thread_rng();

    for i in 0..k {
        for j in 0..n {
            a[i][j] = rng.gen::<Torus>();
        }
        let a_s = convolution_mod(&a[i], &s[i]);
        for j in 0..n {
            //b[j] += a_s[j];
            b[j] = b[j].wrapping_add(a_s[j]);
        }
    }
    for i in 0..n {
        //b[i] += m[i] + e[i];
        b[i] = b[i].wrapping_add(m2[i].wrapping_add(key.key_e[i]));
    }
    TrlweEncryption { a, b }
}

pub fn decrypt(a: &[Vec<Torus>], b: &[Torus], s: &[Vec<Torus>], n: usize, k: usize) -> Vec<i64> {
    let mut b: Vec<Torus> = b.to_vec();
    for i in 0..k {
        let a_s = convolution_mod(&a[i], &s[i]);
        for j in 0..n {
            b[j] = b[j].wrapping_sub(a_s[j]);
        }
    }
    let mut res: Vec<i64> = vec![0; n];
    for i in 0..n {
        if u32::MAX / 2 > b[i] {
            //sgn = 1;
            res[i] = 1;
        } else {
            //sgn = -1;
            res[i] = 0;
        }
    }
    res
}

pub fn test(n: usize, k: usize) {
    let mut bit = vec![0; n];
    let mut rng = rand::thread_rng();
    for i in bit.iter_mut() {
        *i = rng.gen_range(0..2);
    }
    let t: Trlwe = Trlwe::new_trlwe(&bit, n, k);
    let dec = decrypt(&t.enc.a, &t.enc.b, &t.key.key_s, n, k);
    for i in bit.iter_mut().enumerate() {
        if *i.1 != dec[i.0] {
            return;
        }
    }
    println!("TRLWE: OK!");
}
