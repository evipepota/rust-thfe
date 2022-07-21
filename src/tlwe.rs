type Torus = u32;

const ALPHA: f64 = 1.0 / 2i64.pow(15) as f64;
pub const N: usize = 635;

use crate::calc::torus_tool;
use rand::Rng;

pub struct TlweKey {
    pub key_s: Vec<u32>,
    pub key_e: Torus,
}

pub struct TlweEncryption {
    pub a: Vec<Torus>,
    pub b: Torus,
}

impl TlweKey {
    pub fn keygen(size: usize) -> Self {
        //generate s, e
        let e = torus_tool::d_ta(ALPHA);
        let mut rng = rand::thread_rng();
        //let mut s: [u32; N] = [0; N];
        let mut s: Vec<u32> = vec![0; size];
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
    enc: TlweEncryption,
    //dec: i64,
}

impl Tlwe {
    pub fn new_tlwe(bit: i64, size: usize) -> Tlwe {
        //key(s, e),enc(a, b), dec(bit)
        let key = TlweKey::keygen(size);
        let abs = encrypt(bit, &key, size);
        //let ans = decrypt(&abs.a, abs.b, &key.key_s, size);
        Tlwe {
            key: (key),
            enc: abs,
            //dec: (ans),
        }
    }
}

pub fn encrypt(m: i64, key: &TlweKey, size: usize) -> TlweEncryption {
    let mut a: Vec<Torus> = vec![0; size];
    let m = torus_tool::f2torus(((2 * m - 1) as f64) / 8.0);
    let mut b: Torus = 0;
    //let s = key.key_s;
    //let e = key.key_e;

    let mut rng = rand::thread_rng();
    for i in a.iter_mut().enumerate() {
        *i.1 = rng.gen::<Torus>();
        //b += a[i] * s[i];
        b = b.wrapping_add(*i.1 * key.key_s[i.0]);
    }
    //b += m + e;
    b = b.wrapping_add(m.wrapping_add(key.key_e));
    TlweEncryption { a, b }
}

pub fn decrypt(a: &[Torus], b: Torus, s: &[Torus], size: usize) -> i64 {
    let mut b: Torus = b;
    for i in 0..size {
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

pub fn test(bit: i64, size: usize) {
    let t: Tlwe = Tlwe::new_tlwe(bit, size);
    println!("message: {}", bit);
    let i = decrypt(&t.enc.a, t.enc.b, &t.key.key_s, size);
    /*
    print!("encrypt = [");
    for i in 0..size {
        print!("{}, ", t.enc.a[i]);
    }
    print!("{}", t.enc.b);
    println!("]");
    println!();

    println!("e = {}", t.key.key_e);
    println!();

    print!("s = [");
    for i in 0..size {
        print!("{}, ", t.key.key_s[i]);
    }
    println!("]");
    println!();

    println!("decrypt: {}", i);
    */
    
    if i == bit {
        println!("TLWE: OK!")
    }
    println!();
}
