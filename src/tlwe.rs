type Torus = u32;

//const ALPHA: f64 = 1.0 / 2i64.pow(16) as f64;//0.00003051757
const ALPHA: f64 = 0.0000925119974676756;
pub const N: usize = 586;
const K: usize = trlwe::K;

use crate::{calc::torus_tool, trlwe};
use rand::Rng;

pub struct TlweKeylvl1 {
    pub key_s: [Torus; K * trlwe::N],
}

pub struct TlweEncryptionlvl1 {
    pub a: [Torus; K * trlwe::N],
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
    pub fn keygen() -> Self {
        //generate s, e
        let mut rng = rand::thread_rng();
        let mut s: [Torus; K * trlwe::N] = [0; K * trlwe::N];
        for i in s.iter_mut() {
            *i = rng.gen_range(0..2);
        }
        Self { key_s: (s) }
    }
}

pub fn encrypt_torus_lvl1(m: Torus, key: &TlweKeylvl1) -> TlweEncryptionlvl1 {
    let mut a: [Torus; K * trlwe::N] = [0; K * trlwe::N];
    let mut b: Torus = 0;

    let mut rng = rand::thread_rng();
    for i in a.iter_mut().enumerate() {
        *i.1 = rng.gen::<Torus>();
        //b += a[i] * s[i];
        b = b.wrapping_add(*i.1 * key.key_s[i.0]);
    }
    //b += m + e;
    let e = torus_tool::d_ta(trlwe::ALPHA);
    let mut rng = rand::thread_rng();
    b = b.wrapping_add(m.wrapping_add(e));
    TlweEncryptionlvl1 { a, b }
}

pub fn encrypt_bit_lvl1(m: i64, key: &TlweKeylvl1) -> TlweEncryptionlvl1 {
    let mut a: [Torus; K * trlwe::N] = [0; K * trlwe::N];
    let m = torus_tool::f2torus(((2 * m - 1) as f64) / 8.0);
    let mut b: Torus = 0;
    //let s = key.key_s;
    //let e = key.key_e;
    encrypt_torus_lvl1(m, key)
}

pub fn decrypt_lvl1(a: &[Torus], b: Torus, s: &[Torus]) -> i64 {
    let mut b: Torus = b;
    for i in 0..K * trlwe::N {
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
    pub key_s: [Torus; N],
}

impl Copy for TlweEncryptionlvl0 {}

impl Clone for TlweEncryptionlvl0 {
    //Ownership
    fn clone(&self) -> Self {
        *self
    }
}

pub struct TlweEncryptionlvl0 {
    pub a: [Torus; N],
    pub b: Torus,
}

impl TlweKeylvl0 {
    pub fn keygen() -> Self {
        //generate s, e
        let mut rng = rand::thread_rng();
        let mut s: [Torus; N] = [0; N];
        for i in s.iter_mut() {
            *i = rng.gen_range(0..2);
        }
        Self { key_s: (s) }
    }
}

pub fn encrypt_torus_lvl0(m: Torus, key: &TlweKeylvl0) -> TlweEncryptionlvl0 {
    let mut a: [Torus; N] = [0; N];
    let mut b: Torus = 0;

    let mut rng = rand::thread_rng();
    for i in a.iter_mut().enumerate() {
        *i.1 = rng.gen::<Torus>();
        //b += a[i] * s[i];
        b = b.wrapping_add(*i.1 * key.key_s[i.0]);
    }
    //b += m + e;
    let e = torus_tool::d_ta(ALPHA);
    let mut rng = rand::thread_rng();
    b = b.wrapping_add(m.wrapping_add(e));
    TlweEncryptionlvl0 { a, b }
}

pub fn encrypt_bit_lvl0(m: i64, key: &TlweKeylvl0) -> TlweEncryptionlvl0 {
    let mut a: [Torus; N] = [0; N];
    let m = torus_tool::f2torus(((2 * m - 1) as f64) / 8.0);
    let mut b: Torus = 0;
    encrypt_torus_lvl0(m, key)
}

pub fn decrypt_lvl0(a: &[Torus], b: Torus, s: &[Torus]) -> i64 {
    let mut b: Torus = b;
    for i in 0..N {
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

/*pub fn test(bit: i64, size: usize) {
    for i in 0..50 {
    let key = TlweKeylvl0::keygen();
    let t: TlweEncryptionlvl0 = encrypt_bit_lvl0(bit, &key);
    println!("message: {}", bit);
    let i = decrypt_lvl0(&t.a, t.b, &key.key_s);

    if i == bit {
        println!("TLWE: OK!")
    }
    println!();
    }
}*/

/*pub fn test2() {
    let key = TlweKey::keygen(N);
    let mut a0 = encrypt(0, &key, N);
    let a1 = encrypt(1, &key, N);
    for i in 0..N{
        a0.a[i] = a0.a[i].wrapping_add(a1.a[i]);
    }
    a0.b = a0.b.wrapping_add(a1.b);
    let d = decrypt(&a0.a, a0.b, &key.key_s, N);
    println!("{}", d)
}*/
