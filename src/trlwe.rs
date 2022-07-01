use rand::Rng;

use crate::calc::{fft::convolution_mod, torus_tool};

type Torus = u32;
const ALPHA: f64 = 1.0 / 2i64.pow(25) as f64;
pub const N: usize = 512;
pub const K: usize = 2;

pub struct TrlweKey {
    key_s: [[u32; N]; K],
    key_e: [Torus; N],
}

impl Copy for TrlweKey {}

impl Clone for TrlweKey {
    //Ownership
    fn clone(&self) -> TrlweKey {
        *self
    }
}

impl TrlweKey {
    pub fn keygen() -> Self {
        //generate s, e
        let mut e: [Torus; N] = [0; N];
        for i in e.iter_mut() {
            *i = torus_tool::d_ta(ALPHA);
        }
        let mut s: [[u32; N]; K] = [[0; N]; K];
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
    key: TrlweKey,
    enc: ([[Torus; N]; K], [Torus; N]),
    dec: [i64; N],
}

impl Copy for Trlwe {}

impl Clone for Trlwe {
    //Ownership
    fn clone(&self) -> Trlwe {
        *self
    }
}

impl Trlwe {
    //to do
    pub fn new_trlwe(bit: [i64; N]) -> Trlwe {
        let key = TrlweKey::keygen();
        let abs = encrypt(bit, key);
        let ans = decrypt(abs.0, abs.1, key.key_s);
        Trlwe {
            key: (key),
            enc: (abs),
            dec: (ans),
        }
    }
}

#[warn(unused_must_use)]
pub fn encrypt(m: [i64; N], key: TrlweKey) -> ([[Torus; N]; K], [Torus; N]) {
    let mut a: [[Torus; N]; K] = [[0; N]; K];
    let mut m2: [Torus; N] = [0; N];
    for i in 0..N {
        m2[i] = torus_tool::f2torus(((2 * m[i] - 1) as f64) / 8.0);
    }
    let mut b: [Torus; N] = [0; N];
    let s = key.key_s;
    let e = key.key_e;
    let mut rng = rand::thread_rng();

    for i in 0..K {
        for j in 0..N {
            a[i][j] = rng.gen::<Torus>();
        }
        let a_s = convolution_mod(a[i], s[i]);
        for j in 0..N {
            //b[j] += a_s[j];
            b[j] = b[j].wrapping_add(a_s[j]);
        }
    }
    for i in 0..N {
        //b[i] += m[i] + e[i];
        b[i] = b[i].wrapping_add(m2[i].wrapping_add(e[i]));
    }
    (a, b)
}

pub fn decrypt(a: [[Torus; N]; K], b: [Torus; N], s: [[Torus; N]; K]) -> [i64; N] {
    //to do
    let mut b: [Torus; N] = b;
    for i in 0..K {
        let a_s = convolution_mod(a[i], s[i]);
        for j in 0..N {
            b[j] = b[j].wrapping_sub(a_s[j]);
        }
    }
    let mut res: [i64; N] = [0; N];
    for i in 0..N {
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

pub fn test() {
    let mut bit = [0; N];
    let mut rng = rand::thread_rng();
    for i in bit.iter_mut() {
        *i = rng.gen_range(0..2);
    }
    let t: Trlwe = Trlwe::new_trlwe(bit);
    for i in 0..N {
        if bit[i] != t.dec[i] {
            return;
        }
    }
    println!("TRLWE: OK!");
}
