use rand::Rng;

use crate::{
    calc::fft::convolution_mod,
    param,
    sampleextractindex::{sample_extract_index_encryption, sample_extract_index_key},
    tlwe::{
        self, decrypt_lvl0, decrypt_lvl1, encrypt_bit_lvl0, encrypt_bit_lvl1, TlweEncryptionlvl0,
        TlweEncryptionlvl1, TlweKeylvl0, TlweKeylvl1,
    },
    trlwe::{decrypt2, encrypt_bit, zero_encrypt, TrlweEncryption, TrlweKey},
};
use std::time::Instant;

type Torus = u32;
type Trgsw =
    [[[Torus; param::trgsw::N]; (param::trgsw::K + 1) * param::trgsw::L]; param::trgsw::K + 1];
type KeySwitch = Vec<TlweEncryptionlvl0>;

fn trgsw(mu: &[i32], key: &TrlweKey) -> Trgsw {
    let mut trgsw: Trgsw =
        [[[0; param::trgsw::N]; param::trgsw::L * (param::trgsw::K + 1)]; param::trgsw::K + 1];

    for i in 0..(param::trgsw::K + 1) * param::trgsw::L {
        let zero = zero_encrypt(key);
        for h in 0..param::trgsw::N {
            trgsw[0][i][h] = trgsw[0][i][h].wrapping_add(zero.a[0][h]);
            trgsw[1][i][h] = trgsw[1][i][h].wrapping_add(zero.a[1][h]);
            trgsw[2][i][h] = trgsw[2][i][h].wrapping_add(zero.b[h]);
        }
    }

    for i in 0..param::trgsw::L {
        for j in 0..param::trgsw::N {
            let t = (mu[j] * (1 << (32 - (i + 1) * param::trgsw::BGBIT))) as Torus;
            for h in 0..(param::trgsw::K + 1) {
                //trgsw[h][i + param::trgsw::L * h][j] += t;
                trgsw[h][i + param::trgsw::L * h][j] =
                    trgsw[h][i + param::trgsw::L * h][j].wrapping_add(t);
            }
        }
    }
    trgsw
}

fn trgsw_bool(mu: i32, key: &TrlweKey) -> Trgsw {
    let mut dum = [0; param::trgsw::N];
    dum[0] = mu;
    trgsw(&dum, key)
}

fn cmux(c: &Trgsw, out1: &TrlweEncryption, out2: &TrlweEncryption) -> TrlweEncryption {
    let mut out_a: [[Torus; param::trgsw::N]; param::trgsw::K] =
        [[0; param::trgsw::N]; param::trgsw::K];
    let mut out_b: [Torus; param::trgsw::N] = [0; param::trgsw::N];
    for i in 0..param::trgsw::N {
        out_a[0][i] = out1.a[0][i].wrapping_sub(out2.a[0][i]);
        out_a[1][i] = out1.a[1][i].wrapping_sub(out2.a[1][i]);
        out_b[i] = out1.b[i].wrapping_sub(out2.b[i]);
    }
    let t = TrlweEncryption { a: out_a, b: out_b };
    let mut mul = extenalproduct(c, &t);
    for i in 0..param::trgsw::N {
        mul.a[0][i] = mul.a[0][i].wrapping_add(out2.a[0][i]);
        mul.a[1][i] = mul.a[1][i].wrapping_add(out2.a[1][i]);
        mul.b[i] = mul.b[i].wrapping_add(out2.b[i]);
    }
    mul
}

fn extenalproduct(c: &Trgsw, t: &TrlweEncryption) -> TrlweEncryption {
    let mut out_a: [[Torus; param::trgsw::N]; param::trgsw::K] =
        [[0; param::trgsw::N]; param::trgsw::K];
    let mut out_b: [Torus; param::trgsw::N] = [0; param::trgsw::N];
    let dec_a0 = decomposition(&t.a[0]);
    let dec_a1 = decomposition(&t.a[1]);
    let dec_b = decomposition(&t.b);
    for i in 0..param::trgsw::L {
        let ch = convolution_mod(&dec_a0[i], &c[0][i]);
        for j in 0..param::trgsw::N {
            //out_a[0][j] += ch[j];
            out_a[0][j] = out_a[0][j].wrapping_add(ch[j]);
        }
        let ch = convolution_mod(&dec_a1[i], &c[0][param::trgsw::L + i]);
        for j in 0..param::trgsw::N {
            //out_a[0][j] += ch[j];
            out_a[0][j] = out_a[0][j].wrapping_add(ch[j]);
        }
        let ch = convolution_mod(&dec_b[i], &c[0][2 * param::trgsw::L + i]);
        for j in 0..param::trgsw::N {
            //out_a[0][j] += ch[j];
            out_a[0][j] = out_a[0][j].wrapping_add(ch[j]);
        }

        let ch = convolution_mod(&dec_a0[i], &c[1][i]);
        for j in 0..param::trgsw::N {
            //out_a[1][j] += ch[j];
            out_a[1][j] = out_a[1][j].wrapping_add(ch[j]);
        }
        let ch = convolution_mod(&dec_a1[i], &c[1][param::trgsw::L + i]);
        for j in 0..param::trgsw::N {
            //out_a[1][j] += ch[j];
            out_a[1][j] = out_a[1][j].wrapping_add(ch[j]);
        }
        let ch = convolution_mod(&dec_b[i], &c[1][2 * param::trgsw::L + i]);
        for j in 0..param::trgsw::N {
            //out_a[1][j] += ch[j];
            out_a[1][j] = out_a[1][j].wrapping_add(ch[j]);
        }

        let ch = convolution_mod(&dec_a0[i], &c[2][i]);
        for j in 0..param::trgsw::N {
            //out_b[j] += ch[j];
            out_b[j] = out_b[j].wrapping_add(ch[j]);
        }
        let ch = convolution_mod(&dec_a1[i], &c[2][param::trgsw::L + i]);
        for j in 0..param::trgsw::N {
            //out_b[j] += ch[j];
            out_b[j] = out_b[j].wrapping_add(ch[j]);
        }
        let ch = convolution_mod(&dec_b[i], &c[2][2 * param::trgsw::L + i]);
        for j in 0..param::trgsw::N {
            //out_b[j] += ch[j];
            out_b[j] = out_b[j].wrapping_add(ch[j]);
        }
    }

    TrlweEncryption { a: out_a, b: out_b }
}

fn decomposition(a: &[Torus]) -> [[u32; param::trgsw::N]; param::trgsw::L] {
    let mut offset: u32 = 0;
    for i in 0..param::trgsw::L {
        offset =
            offset.wrapping_add(param::trgsw::BG / 2 * (1 << (32 - (i + 1) * param::trgsw::BGBIT)));
    }
    let mut tmp = [0; param::trgsw::N];
    for i in 0..param::trgsw::N {
        tmp[i] = a[i].wrapping_add(offset);
    }
    let mut ans = [[0; param::trgsw::N]; param::trgsw::L];
    for i in 0..param::trgsw::L {
        for j in 0..param::trgsw::N {
            ans[i][j] = (((tmp[j]) >> (32 - (param::trgsw::BGBIT as u32 * ((i as u32) + 1))))
                & (param::trgsw::BG - 1))
                .wrapping_sub(param::trgsw::BG / 2);
        }
    }
    ans
}

fn blind_rotate(src: &TlweEncryptionlvl0, bk: &[Trgsw], tv: &TrlweEncryption) -> TrlweEncryption {
    let b_bar = 2 * param::trlwe::N
        - (((src.b as usize) + (1 << (31 - param::trgsw::NBIT - 1)))
            >> (32 - param::trgsw::NBIT - 1));
    let mut res_a = [[0; param::trgsw::N]; param::trgsw::K];
    res_a[0] = x_k(&tv.a[0], b_bar);
    res_a[1] = x_k(&tv.a[1], b_bar);
    let mut res = TrlweEncryption {
        a: res_a,
        b: x_k(&tv.b, b_bar),
    };

    for i in 0..param::tlwe_lvl0::N {
        let a_bar = (src.a[i].wrapping_add(1 << (31 - param::trgsw::NBIT - 1))
            >> (32 - param::trgsw::NBIT - 1)) as usize;
        let mut res2_a = [[0; param::trgsw::N]; param::trgsw::K];

        res2_a[0] = x_k(&res.a[0], a_bar);
        res2_a[1] = x_k(&res.a[1], a_bar);
        let res2 = TrlweEncryption {
            a: res2_a,
            b: x_k(&res.b, a_bar),
        };

        res = cmux(&bk[i], &res2, &res); //ok
    }
    res
}

pub fn x_k(a: &[Torus], k: usize) -> [Torus; param::trgsw::N] {
    let mut res = [0; param::trgsw::N];

    if k < param::trgsw::N {
        res[k..((param::trgsw::N - k) + k)].copy_from_slice(&a[..(param::trgsw::N - k)]);
        /*for i in 0..(param::trgsw::N - k) {
            res[i + k] = a[i];
        }*/
        for i in (param::trgsw::N - k)..param::trgsw::N {
            res[i + k - param::trgsw::N] = u32::MAX - a[i];
        }
    } else {
        for i in 0..2 * param::trgsw::N - k {
            res[i + k - param::trgsw::N] = u32::MAX - a[i];
        }
        for i in (2 * param::trgsw::N - k)..param::trgsw::N {
            res[i - (2 * param::trgsw::N - k)] = a[i];
        }
    }

    res
}

fn gate_bootstrapping_tlwe2tlwe(src: &TlweEncryptionlvl0, bk: &[Trgsw]) -> TlweEncryptionlvl1 {
    const MU: Torus = 1 << 29;
    let res_a = [[0; param::trgsw::N]; param::trgsw::K];
    let res_b = [MU; param::trgsw::N];
    let res = TrlweEncryption { a: res_a, b: res_b };
    let trlwe = blind_rotate(src, bk, &res);
    sample_extract_index_encryption(&trlwe, 0)
}


fn bootstrappingkey_gen(key: &TlweKeylvl0, key2: &TrlweKey) -> Vec<Trgsw> {
    let trgsw_dum: Trgsw =
        [[[0; param::trgsw::N]; param::trgsw::L * (param::trgsw::K + 1)]; param::trgsw::K + 1];
    let mut res = vec![trgsw_dum; param::tlwe_lvl0::N]; //size...
    for i in 0..param::tlwe_lvl0::N {
        res[i] = trgsw_bool(key.key_s[i] as i32, key2);
    }
    res
}

fn keyswtching_gen(key1: &TlweKeylvl1, key0: &TlweKeylvl0) -> KeySwitch {
    const KN: usize = param::trgsw::K * param::trgsw::N; //k*n
    let dummy = TlweEncryptionlvl0 {
        a: [0; param::tlwe_lvl0::N],
        b: 0,
    };
    let mut res = vec![dummy; param::trgsw::BASE * param::trgsw::T * KN];
    for i in 0..KN {
        for j in 0..param::trgsw::T {
            for o in 1..param::trgsw::BASE {
                let p = (o as u32) * key1.key_s[i] * (1 << (32 - param::trgsw::BASEBIT * (j + 1)));
                let idx = (param::trgsw::BASE * param::trgsw::T * i) + (param::trgsw::BASE * j) + o;
                res[idx] = tlwe::encrypt_torus_lvl0(p, key0);
            }
        }
    }
    res
}

fn identity_key_swtching(ab: TlweEncryptionlvl1, ks: &KeySwitch) -> TlweEncryptionlvl0 {
    let mut res = TlweEncryptionlvl0 {
        a: [0; param::tlwe_lvl0::N],
        b: 0,
    };
    res.b = ab.b;

    let prec_offset = 1 << (32 - (1 + param::trgsw::BASEBIT * param::trgsw::T));
    for i in 0..(param::trgsw::K * param::trgsw::N) {
        let a_bar = ab.a[i].wrapping_add(prec_offset);
        for j in 0..param::trgsw::T {
            let k =
                (a_bar >> (32 - (j + 1) * param::trgsw::BASEBIT)) & (param::trgsw::BASE - 1) as u32;
            if k != 0 {
                let idx = (param::trgsw::BASE * param::trgsw::T * i)
                    + (param::trgsw::BASE * j)
                    + k as usize;
                for o in 0..param::tlwe_lvl0::N {
                    res.a[o] = res.a[o].wrapping_sub(ks[idx].a[o]);
                }
                res.b = res.b.wrapping_sub(ks[idx].b);
            }
        }
    }
    res
}

pub fn homnand(
    a0: TlweEncryptionlvl0,
    a1: TlweEncryptionlvl0,
    bk: &[Trgsw],
    ks: &KeySwitch,
) -> TlweEncryptionlvl0 {
    const MU: Torus = 1 << 29;
    let mut res = TlweEncryptionlvl0 {
        a: [0; param::tlwe_lvl0::N],
        b: MU,
    };
    for i in 0..param::tlwe_lvl0::N {
        res.a[i] = res.a[i].wrapping_sub(a0.a[i]);
        res.a[i] = res.a[i].wrapping_sub(a1.a[i]);
    }
    res.b = res.b.wrapping_sub(a0.b);
    res.b = res.b.wrapping_sub(a1.b);

    let t1 = gate_bootstrapping_tlwe2tlwe(&res, bk);

    identity_key_swtching(t1, ks)
}

pub fn nand_test() {
    //key_gen
    let sk_tlwe = TlweKeylvl0::keygen();
    let sk_trlwe = TrlweKey::keygen();
    let bk = bootstrappingkey_gen(&sk_tlwe, &sk_trlwe);

    let sk_trlew2tlwelvl1 = sample_extract_index_key(&sk_trlwe);

    let ks = keyswtching_gen(&sk_trlew2tlwelvl1, &sk_tlwe);
    for _i in 0..30 {
        let start = Instant::now();
        let mut rng = rand::thread_rng();
        let l = rng.gen_range(0..2);
        let r = rng.gen_range(0..2);
        let lhs = encrypt_bit_lvl0(l, &sk_tlwe);
        let rhs = encrypt_bit_lvl0(r, &sk_tlwe);

        let res = homnand(lhs, rhs, &bk, &ks);
        let ans = decrypt_lvl0(&res.a, res.b, &sk_tlwe.key_s);
        let end = start.elapsed();
        println!("{} NAND {} = {}", l, r, ans);
        println!("{}{:03}ms", end.as_secs(), end.subsec_millis());
    }
}

#[allow(dead_code)]
pub fn test_cmux() {
    let b1 = [0; param::trgsw::N];
    let b2 = [1; param::trgsw::N];
    let mut c = [0; param::trgsw::N];
    c[0] = 1;

    let key = TrlweKey::keygen();
    let a1 = encrypt_bit(&b1, &key);
    let a2 = encrypt_bit(&b2, &key);
    let cc = trgsw(&c, &key);
    let k = cmux(&cc, &a1, &a2);
    let u = decrypt2(&k.a, &k.b, &key.key_s);
    for i in u.iter().take(param::trgsw::N) {
        println!("{}", i);
    }
}

#[allow(dead_code)]
pub fn test_blindrotate() {
    let sk_tlwe = TlweKeylvl0::keygen();
    let sk_trlwe = TrlweKey::keygen();
    let bk = bootstrappingkey_gen(&sk_tlwe, &sk_trlwe);
    let plain = 0;
    let tl = encrypt_bit_lvl0(plain, &sk_tlwe);

    const MU: Torus = 1 << 29;
    let res_a = [[0; param::trgsw::N]; param::trgsw::K];
    let res_b = [MU; param::trgsw::N];
    let res = TrlweEncryption { a: res_a, b: res_b };

    let trlwe = blind_rotate(&tl, &bk, &res);
    let tl1 = sample_extract_index_encryption(&trlwe, 0);
    let k2 = sample_extract_index_key(&sk_trlwe);
    let d = decrypt_lvl1(&tl1.a, tl1.b, &k2.key_s);
    println!("{}, {}", d, plain);
}

#[allow(dead_code)]
pub fn test_identity_key_swwitch() {
    let p = 1;
    for _i in 0..30 {
        let sk_tlwe = TlweKeylvl0::keygen();
        let sk_trlwe = TrlweKey::keygen();
        let sk_trlew2tlwelvl1 = sample_extract_index_key(&sk_trlwe);
        let ks = keyswtching_gen(&sk_trlew2tlwelvl1, &sk_tlwe);

        let tlwe1 = encrypt_bit_lvl1(p, &sk_trlew2tlwelvl1);
        let tlwe0 = identity_key_swtching(tlwe1, &ks);
        let d1 = decrypt_lvl1(&tlwe1.a, tlwe1.b, &sk_trlew2tlwelvl1.key_s);
        let d0 = decrypt_lvl0(&tlwe0.a, tlwe0.b, &sk_tlwe.key_s);
        println!("{}, {}", d0, d1);
    }
}
