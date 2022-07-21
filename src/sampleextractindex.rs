type Torus = u32;

use crate::{
    tlwe::{TlweEncryption, TlweKey},
    trlwe::{TrlweEncryption, TrlweKey},
};

pub fn sample_extract_index_encryption(ab: &TrlweEncryption, x: usize) -> TlweEncryption {
    let s = ab.a.len() * ab.b.len(); // k * n
    let n = ab.b.len();
    let k = ab.a.len();
    let mut a: Vec<Torus> = vec![0; s];
    let b = ab.b[x];
    for j in 0..k {
        for i in 0..x + 1 {
            a[i + n * j] = ab.a[j][i];
        }
        for i in x + 1..n {
            a[i + n * j] = u32::MIN.wrapping_sub(ab.a[j][i]);
        }
    }

    TlweEncryption { a, b }
}
pub fn sample_extract_index_key(se: &TrlweKey, x: usize) -> TlweKey {
    let size = se.key_s.len() * se.key_e.len(); // k * n
    let n = se.key_e.len();
    let k = se.key_s.len();
    let mut s: Vec<Torus> = vec![0; size];
    let e = se.key_e[x];
    for j in 0..k {
        for i in 0..n {
            s[i + n * j] = se.key_s[j][i];
        }
    }

    TlweKey { key_s: s, key_e: e }
}
