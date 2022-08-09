use rand::Rng;

use crate::{
    calc::{fft::convolution_mod, torus_tool},
    sampleextractindex::{sample_extract_index_encryption, sample_extract_index_key},
    tlwe::decrypt_lvl1,
};

type Torus = u32;
//pub const ALPHA: f64 = 1.0 / 2i64.pow(26) as f64;
pub const ALPHA: f64 = 0.0000000342338787018369;
pub const N: usize = 512;
pub const K: usize = 2;

pub struct TrlweKey {
    pub key_s: [[Torus; N]; K],
}

pub struct TrlweEncryption {
    pub a: [[Torus; N]; K],
    pub b: [Torus; N],
}

impl TrlweKey {
    pub fn keygen() -> Self {
        //generate s, e
        let mut s: [[Torus; N]; K] = [[0; N]; K];
        let mut rng = rand::thread_rng();
        for i in s.iter_mut() {
            for j in i.iter_mut() {
                *j = rng.gen_range(0..2);
            }
        }
        Self { key_s: (s) }
    }
}

pub struct Trlwe {
    pub key: TrlweKey,
    pub enc: TrlweEncryption,
}

pub fn encrypt_torus(m: &[Torus], key: &TrlweKey) -> TrlweEncryption {
    let mut a: [[Torus; N]; K] = [[0; N]; K];
    let mut b: [Torus; N] = [0; N];
    let s = &key.key_s;
    let mut rng = rand::thread_rng();

    for i in 0..K {
        for j in 0..N {
            a[i][j] = rng.gen::<Torus>();
        }
        let a_s = convolution_mod(&a[i], &s[i]);
        //let a_s2 = fft_test(&a[i], &s[i]);
        for j in 0..N {
            //b[j] += a_s[j];
            b[j] = b[j].wrapping_add(a_s[j]);
        }
    }
    for i in 0..N {
        //b[i] += m[i] + e[i];
        let e = torus_tool::d_ta(ALPHA);
        b[i] = b[i].wrapping_add(m[i].wrapping_add(e));
    }
    TrlweEncryption { a, b }
}

pub fn encrypt_bit(m: &[i64], key: &TrlweKey) -> TrlweEncryption {
    let mut m2: [Torus; N] = [0; N];
    for i in 0..N {
        m2[i] = torus_tool::f2torus(((2 * m[i] - 1) as f64) / 8.0);
    }
    encrypt_torus(&m2, key)
}

pub fn zero_encrypt(key: &TrlweKey) -> TrlweEncryption {
    let m: [Torus; N] = [0; N];
    encrypt_torus(&m, key)
}

/*pub fn decrypt(a: &[[Torus; N]; K], b: &[Torus], s: &[[Torus; N]; K]) -> [i64; N] {
    let mut b_bar = [0; N];
    /*for i in 0..N {
        b_bar[i] = b[i];
    }*/
    b_bar[..N].copy_from_slice(&b[..N]);
    for i in 0..K {
        let a_s = fft_test(&a[i], &s[i]);
        for j in 0..N {
            b_bar[j] = b_bar[j].wrapping_sub(a_s[j]);
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
}*/

pub fn decrypt2(a: &[[Torus; N]; K], b: &[Torus], s: &[[Torus; N]; K]) -> Vec<i64> {
    let mut b: Vec<Torus> = b.to_vec();
    for i in 0..K {
        let a_s = convolution_mod(&a[i], &s[i]);
        for j in 0..N {
            //println!("{}, {}", a_s[j], ca_s[j]);
            b[j] = b[j].wrapping_sub(a_s[j]);
        }
    }
    let mut res: Vec<i64> = vec![0; N];
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
    for l in 0..30 {
        let mut bit = [0; N];
        let mut rng = rand::thread_rng();
        for i in bit.iter_mut() {
            *i = rng.gen_range(0..2);
        }
        let key = TrlweKey::keygen();
        let t: TrlweEncryption = encrypt_bit(&bit, &key);
        let dec = decrypt2(&t.a, &t.b, &key.key_s);
        for i in bit.iter_mut().enumerate() {
            if *i.1 != dec[i.0] {
                println!("{}, {}", *i.1, dec[i.0]);
                return;
            }
        }

        let d = sample_extract_index_encryption(&t, 0);
        let kk = sample_extract_index_key(&key, 0);
        let o = decrypt_lvl1(&d.a, d.b, &kk.key_s);

        println!("{}, {}", o, bit[0]);
    }

    println!("TRLWE: OK!");
}

/*pub fn test2() {
    let k = TrlweKey::keygen();
    let bit0 = vec![1; N];
    let mut e0 = encrypt(&bit0, &k);
    for _l in 0..10 {
        let e1 = zero_encrypt(&k);
        for i in 0..N {
            e0.a[0][i] = e0.a[0][i].wrapping_add(e1.a[0][i]);
            e0.a[1][i] = e0.a[1][i].wrapping_add(e1.a[1][i]);
            e0.b[i] = e0.b[i].wrapping_add(e1.b[i]);
        }
    }
    let d = decrypt(&e0.a, &e0.b, &k.key_s);
    for i in 0..N {
        print!("{}", d[i]);
    }
}*/
