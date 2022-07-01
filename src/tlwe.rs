type Torus = u32;

const ALPHA: f64 = 1.0 / 2i64.pow(15) as f64;
const N: usize = 635;

use crate::calc::torus_tool;
use rand::Rng;

pub struct TlweKey {
    key_s: [u32; N],
    key_e: Torus,
}

impl Copy for TlweKey {}

impl Clone for TlweKey {
    //Ownership
    fn clone(&self) -> TlweKey {
        *self
    }
}

impl TlweKey {
    pub fn keygen() -> Self {
        //generate s, e
        let e = torus_tool::d_ta(ALPHA);
        let mut rng = rand::thread_rng();
        let mut s: [u32; N] = [0; N];
        for i in s.iter_mut() {
            *i = rng.gen_range(0..2);
        }
        Self {
            key_s: (s),
            key_e: (e),
        }
    }
}

struct Tlwe {
    key: TlweKey,
    enc: ([Torus; N], Torus),
    dec: i64,
}

impl Copy for Tlwe {}

impl Clone for Tlwe {
    //Ownership
    fn clone(&self) -> Tlwe {
        *self
    }
}

impl Tlwe {
    pub fn new_tlwe(bit: i64) -> Tlwe {
        //key(s, e),enc(a, b), dec(bit) 
        let key = TlweKey::keygen();
        let abs = encrypt(bit, key);
        let ans = decrypt(abs.0, abs.1, key.key_s);
        Tlwe {
            key: (key),
            enc: (abs),
            dec: (ans),
        }
    }
}

#[warn(unused_must_use)]
pub fn encrypt(m: i64, key: TlweKey) -> ([Torus; N], Torus) {
    let mut a: [Torus; N] = [0; N];
    let m = torus_tool::f2torus(((2 * m - 1) as f64) / 8.0);
    let mut b: Torus = 0;
    let s = key.key_s;
    let e = key.key_e;

    let mut rng = rand::thread_rng();
    for i in 0..N {
        a[i] = rng.gen::<Torus>();
        //b += a[i] * s[i];
        b = b.wrapping_add(a[i] * s[i]);
    }
    //b += m + e;
    b = b.wrapping_add(m.wrapping_add(e));
    (a, b)
}

pub fn decrypt(a: [Torus; N], b: Torus, s: [Torus; N]) -> i64 {
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

pub fn test(bit: i64) {
    let t: Tlwe = Tlwe::new_tlwe(bit);
    println!("message: {}", bit);
/*
    print!("encrypt = [");
    for i in 0..N {
        print!("{}, ", t.enc.0[i]);
    }
    print!("{}", t.enc.1);
    println!("]");
    println!();

    println!("e = {}", t.key.key_e);
    println!();

    print!("s = [");
    for i in 0..N {
        print!("{}, ", t.key.key_s[i]);
    }
    println!("]");
    println!();
*/
    println!("decrypt: {}", t.dec);
    if t.dec == bit {
        println!("TLWE: OK!")
    }
    println!();
}
