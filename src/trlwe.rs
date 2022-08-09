use rand::Rng;

use crate::{
    calc::{fft::convolution_mod, torus_tool},
    param,
    sampleextractindex::{sample_extract_index_encryption, sample_extract_index_key},
    tlwe::decrypt_lvl1,
};

type Torus = u32;

pub struct TrlweKey {
    pub key_s: [[Torus; param::trlwe::N]; param::trlwe::K],
}

pub struct TrlweEncryption {
    pub a: [[Torus; param::trlwe::N]; param::trlwe::K],
    pub b: [Torus; param::trlwe::N],
}

impl TrlweKey {
    pub fn keygen() -> Self {
        //generate s, e
        let mut s: [[Torus; param::trlwe::N]; param::trlwe::K] =
            [[0; param::trlwe::N]; param::trlwe::K];
        let mut rng = rand::thread_rng();
        for i in s.iter_mut() {
            for j in i.iter_mut() {
                *j = rng.gen_range(0..2);
            }
        }
        Self { key_s: (s) }
    }
}

pub fn encrypt_torus(m: &[Torus], key: &TrlweKey) -> TrlweEncryption {
    let mut a: [[Torus; param::trlwe::N]; param::trlwe::K] =
        [[0; param::trlwe::N]; param::trlwe::K];
    let mut b: [Torus; param::trlwe::N] = [0; param::trlwe::N];
    let s = &key.key_s;
    let mut rng = rand::thread_rng();

    for i in 0..param::trlwe::K {
        for j in 0..param::trlwe::N {
            a[i][j] = rng.gen::<Torus>();
        }
        let a_s = convolution_mod(&a[i], &s[i]);
        for j in 0..param::trlwe::N {
            //b[j] += a_s[j];
            b[j] = b[j].wrapping_add(a_s[j]);
        }
    }
    for i in 0..param::trlwe::N {
        //b[i] += m[i] + e[i];
        let e = torus_tool::d_ta(param::trlwe::ALPHA);
        b[i] = b[i].wrapping_add(m[i].wrapping_add(e));
    }
    TrlweEncryption { a, b }
}

pub fn encrypt_bit(m: &[i64], key: &TrlweKey) -> TrlweEncryption {
    let mut m2: [Torus; param::trlwe::N] = [0; param::trlwe::N];
    for i in 0..param::trlwe::N {
        m2[i] = torus_tool::f2torus(((2 * m[i] - 1) as f64) / 8.0);
    }
    encrypt_torus(&m2, key)
}

pub fn zero_encrypt(key: &TrlweKey) -> TrlweEncryption {
    let m: [Torus; param::trlwe::N] = [0; param::trlwe::N];
    encrypt_torus(&m, key)
}

pub fn decrypt2(
    a: &[[Torus; param::trlwe::N]; param::trlwe::K],
    b: &[Torus],
    s: &[[Torus; param::trlwe::N]; param::trlwe::K],
) -> Vec<i64> {
    let mut b: Vec<Torus> = b.to_vec();
    for i in 0..param::trlwe::K {
        let a_s = convolution_mod(&a[i], &s[i]);
        for j in 0..param::trlwe::N {
            b[j] = b[j].wrapping_sub(a_s[j]);
        }
    }
    let mut res: Vec<i64> = vec![0; param::trlwe::N];
    for i in 0..param::trlwe::N {
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

#[allow(dead_code)]
pub fn test_trlwe() {
    for _l in 0..30 {
        let mut bit = [0; param::trlwe::N];
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
        let kk = sample_extract_index_key(&key);
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
