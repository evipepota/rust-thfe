type Torus = u32;

use crate::{calc::torus_tool, param};
use rand::Rng;

pub struct TlweKeylvl1 {
    pub key_s: [Torus; param::trlwe::K * param::trlwe::N],
}

pub struct TlweEncryptionlvl1 {
    pub a: [Torus; param::trlwe::K * param::trlwe::N],
    pub b: Torus,
}

impl Copy for TlweEncryptionlvl1 {}

impl Clone for TlweEncryptionlvl1 {
    //Ownership
    fn clone(&self) -> Self {
        *self
    }
}

impl TlweKeylvl1 {
    #[allow(dead_code)]
    pub fn keygen() -> Self {
        //generate s, e
        let mut rng = rand::thread_rng();
        let mut s: [Torus; param::trlwe::K * param::trlwe::N] =
            [0; param::trlwe::K * param::trlwe::N];
        for i in s.iter_mut() {
            *i = rng.gen_range(0..2);
        }
        Self { key_s: (s) }
    }
}

pub fn encrypt_torus_lvl1(m: Torus, key: &TlweKeylvl1) -> TlweEncryptionlvl1 {
    let mut a: [Torus; param::trlwe::K * param::trlwe::N] = [0; param::trlwe::K * param::trlwe::N];
    let mut b: Torus = 0;

    let mut rng = rand::thread_rng();
    for i in a.iter_mut().enumerate() {
        *i.1 = rng.gen::<Torus>();
        //b += a[i] * s[i];
        b = b.wrapping_add(*i.1 * key.key_s[i.0]);
    }
    //b += m + e;
    let e = torus_tool::d_ta(param::trlwe::ALPHA);
    b = b.wrapping_add(m.wrapping_add(e));
    TlweEncryptionlvl1 { a, b }
}

#[allow(dead_code)]
pub fn encrypt_bit_lvl1(m: i64, key: &TlweKeylvl1) -> TlweEncryptionlvl1 {
    let m = torus_tool::f2torus(((2 * m - 1) as f64) / 8.0);
    encrypt_torus_lvl1(m, key)
}

#[allow(dead_code)]
pub fn decrypt_lvl1(a: &[Torus], b: Torus, s: &[Torus]) -> i64 {
    let mut b: Torus = b;
    for i in 0..(param::trlwe::K * param::trlwe::N) {
        //b -= a[i] * s[i];
        b = b.wrapping_sub(a[i] * s[i]);
    }
    if u32::MAX / 2 > b {
        //sgn = 1;
        1
    } else {
        //sgn = -1;
        0
    }
}

pub struct TlweKeylvl0 {
    pub key_s: [Torus; param::tlwe_lvl0::N],
}

impl Copy for TlweEncryptionlvl0 {}

impl Clone for TlweEncryptionlvl0 {
    //Ownership
    fn clone(&self) -> Self {
        *self
    }
}

pub struct TlweEncryptionlvl0 {
    pub a: [Torus; param::tlwe_lvl0::N],
    pub b: Torus,
}

impl TlweKeylvl0 {
    pub fn keygen() -> Self {
        //generate s, e
        let mut rng = rand::thread_rng();
        let mut s: [Torus; param::tlwe_lvl0::N] = [0; param::tlwe_lvl0::N];
        for i in s.iter_mut() {
            *i = rng.gen_range(0..2);
        }
        Self { key_s: (s) }
    }
}

pub fn encrypt_torus_lvl0(m: Torus, key: &TlweKeylvl0) -> TlweEncryptionlvl0 {
    let mut a: [Torus; param::tlwe_lvl0::N] = [0; param::tlwe_lvl0::N];
    let mut b: Torus = 0;

    let mut rng = rand::thread_rng();
    for i in a.iter_mut().enumerate() {
        *i.1 = rng.gen::<Torus>();
        //b += a[i] * s[i];
        b = b.wrapping_add(*i.1 * key.key_s[i.0]);
    }
    //b += m + e;
    let e = torus_tool::d_ta(param::tlwe_lvl0::ALPHA);
    b = b.wrapping_add(m.wrapping_add(e));
    TlweEncryptionlvl0 { a, b }
}

pub fn encrypt_bit_lvl0(m: i64, key: &TlweKeylvl0) -> TlweEncryptionlvl0 {
    let m = torus_tool::f2torus(((2 * m - 1) as f64) / 8.0);
    encrypt_torus_lvl0(m, key)
}

pub fn decrypt_lvl0(a: &[Torus], b: Torus, s: &[Torus]) -> i64 {
    let mut b: Torus = b;
    for i in 0..param::tlwe_lvl0::N {
        //b -= a[i] * s[i];
        b = b.wrapping_sub(a[i] * s[i]);
    }
    if u32::MAX / 2 > b {
        //sgn = 1;
        1
    } else {
        //sgn = -1;
        0
    }
}

#[allow(dead_code)]
pub fn test(bit: i64) {
    for _i in 0..50 {
        let key = TlweKeylvl0::keygen();
        let t: TlweEncryptionlvl0 = encrypt_bit_lvl0(bit, &key);
        println!("message: {}", bit);
        let i = decrypt_lvl0(&t.a, t.b, &key.key_s);

        if i == bit {
            println!("TLWE: OK!")
        }
        println!();
    }
}