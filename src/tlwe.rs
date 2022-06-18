type Torus = u32;

const A: f64 = 1.0 / 2i64.pow(15) as f64;
const N: usize = 635;

use rand::prelude::{thread_rng, Distribution};
use rand::Rng;

fn f2torus(src: f64) -> Torus {
    //f64 -> i32
    let mut seisuu: f64 = src - src.round();
    if seisuu < 0.0 {
        seisuu += 1.0;
    }
    (seisuu * (u32::MAX as f64)) as u32
}

fn d_ta() -> Torus {
    //choose from modular normal distribution
    let mut rng = thread_rng();
    let dist = rand_distr::Normal::<f64>::new(0.0, A).unwrap();
    f2torus(dist.sample(&mut rng))
}

pub struct TlweKey {
    key_s: [Torus; N],
    key_e: Torus,
}

impl Copy for TlweKey {}

impl Clone for TlweKey {
    fn clone(&self) -> TlweKey {
        *self
    }
}

impl TlweKey {
    pub fn keygen() -> TlweKey {
        //s, e
        let e = d_ta();
        let mut rng = rand::thread_rng();
        let mut s: [Torus; N] = [0; N];
        for i in 0..N {
            s[i] = rng.gen_range(0..2);
        }
        TlweKey {
            key_s: (s),
            key_e: (e),
        }
    }
}

#[warn(unused_must_use)]
pub fn encrypt(m: i64, key: TlweKey) -> ([Torus; N], Torus) {
    let mut a: [Torus; N] = [0; N];
    let m = f2torus(((2 * m - 1) as f64) / 8.0);
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

pub fn decrypt(a: [Torus; N], b: Torus, s: [Torus; N]) -> Torus {
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

struct Tlwe {
    key: TlweKey,
    enc: ([Torus; N], Torus),
    dec: Torus,
}

impl Copy for Tlwe {}

impl Clone for Tlwe {
    fn clone(&self) -> Tlwe {
        *self
    }
}

impl Tlwe {
    pub fn new_tlwe(bit: i64) -> Tlwe {
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

pub fn test(bit: i64) {
    let t: Tlwe = Tlwe::new_tlwe(bit);
    println!("message: {}", bit);
    println!();

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

    println!("decrypt: {}", t.dec);
}