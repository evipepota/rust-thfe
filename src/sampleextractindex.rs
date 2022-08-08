type Torus = u32;

use crate::{
    tlwe::{self, TlweEncryptionlvl1, TlweKeylvl1},
    trlwe::{self, TrlweEncryption, TrlweKey},
};

pub fn sample_extract_index_encryption(ab: &TrlweEncryption, x: usize) -> TlweEncryptionlvl1 {
    const n: usize = trlwe::N;
    const k: usize = trlwe::K;
    let mut a: [Torus; k * n] = [0; k * n];
    let b = ab.b[x];
    for j in 0..k {
        for i in 0..x + 1 {
            a[i + n * j] = ab.a[j][x - i];
        }
        for i in x + 1..n {
            a[i + n * j] = u32::MIN.wrapping_sub(ab.a[j][n + x - i]);
        }
    }
    TlweEncryptionlvl1 { a, b }
}

pub fn sample_extract_index_key(se: &TrlweKey, x: usize) -> TlweKeylvl1 {
    const n: usize = trlwe::N;
    const k: usize = trlwe::K;
    let mut s: [Torus; k * n] = [0; k * n];
    let e = se.key_e[x];
    for j in 0..k {
        for i in 0..n {
            s[i + n * j] = se.key_s[j][i];
        }
    }

    TlweKeylvl1 { key_s: s, key_e: e }
}
