type Torus = u32;

use crate::{
    param,
    tlwe::{TlweEncryptionlvl1, TlweKeylvl1},
    trlwe::{TrlweEncryption, TrlweKey},
};

pub fn sample_extract_index_encryption(ab: &TrlweEncryption, x: usize) -> TlweEncryptionlvl1 {
    const N: usize = param::trlwe::N;
    const K: usize = param::trlwe::K;
    let mut a: [Torus; K * N] = [0; K * N];
    let b = ab.b[x];
    for j in 0..K {
        for i in 0..x + 1 {
            a[i + N * j] = ab.a[j][x - i];
        }
        for i in x + 1..N {
            a[i + N * j] = u32::MIN.wrapping_sub(ab.a[j][N + x - i]);
        }
    }
    TlweEncryptionlvl1 { a, b }
}

pub fn sample_extract_index_key(se: &TrlweKey) -> TlweKeylvl1 {
    const N: usize = param::trlwe::N;
    const K: usize = param::trlwe::K;
    let mut s: [Torus; K * N] = [0; K * N];
    for j in 0..K {
        for i in 0..N {
            s[i + N * j] = se.key_s[j][i];
        }
    }

    TlweKeylvl1 { key_s: s }
}
