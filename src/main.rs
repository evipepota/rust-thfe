mod calc;
mod key;
mod param;
mod sampleextractindex;
mod tlwe;
mod trgsw;
mod trlwe;
use calc::{
    complex::{self, init},
    fft::{convolution_mod, fft_test},
};

use key::{Cloudkey, Secretkey};
use rand::Rng;
use std::time::Instant;

use crate::{
    tlwe::{decrypt_lvl0, encrypt_bit_lvl0},
    trgsw::{homnand, homor, homand, homxor},
};

fn main() {
    init();
    //nand_test();
    //or_test();
    //and_test();
    xor_test();
    //fft_testtest();
}

pub fn nand_test() {
    //key_gen
    let sk: Secretkey = Secretkey::secretkey_gen();
    let ck: Cloudkey = Cloudkey::cloudkey_gen(&sk);

    for _i in 0..30 {
        let start = Instant::now();
        let mut rng = rand::thread_rng();
        let l = rng.gen_range(0..2);
        let r = rng.gen_range(0..2);
        let lhs = encrypt_bit_lvl0(l, &sk.key_tlwelvl0);
        let rhs = encrypt_bit_lvl0(r, &sk.key_tlwelvl0);
        let res = homnand(lhs, rhs, &ck.key_bootstrap, &ck.key_keyswitch);
        let ans = decrypt_lvl0(&res.a, res.b, &sk.key_tlwelvl0.key_s);
        let end = start.elapsed();
        println!("{} NAND {} = {}", l, r, ans);
        println!("{}{:03}ms", end.as_secs(), end.subsec_millis());
    }
}

pub fn or_test() {
    //key_gen
    let sk: Secretkey = Secretkey::secretkey_gen();
    let ck: Cloudkey = Cloudkey::cloudkey_gen(&sk);

    for _i in 0..30 {
        let start = Instant::now();
        let mut rng = rand::thread_rng();
        let l = rng.gen_range(0..2);
        let r = rng.gen_range(0..2);
        let lhs = encrypt_bit_lvl0(l, &sk.key_tlwelvl0);
        let rhs = encrypt_bit_lvl0(r, &sk.key_tlwelvl0);
        let res = homor(lhs, rhs, &ck.key_bootstrap, &ck.key_keyswitch);
        let ans = decrypt_lvl0(&res.a, res.b, &sk.key_tlwelvl0.key_s);
        let end = start.elapsed();
        println!("{} OR {} = {}", l, r, ans);
        println!("{}{:03}ms", end.as_secs(), end.subsec_millis());
    }
}

pub fn and_test() {
    //key_gen
    let sk: Secretkey = Secretkey::secretkey_gen();
    let ck: Cloudkey = Cloudkey::cloudkey_gen(&sk);

    for _i in 0..30 {
        let start = Instant::now();
        let mut rng = rand::thread_rng();
        let l = rng.gen_range(0..2);
        let r = rng.gen_range(0..2);
        let lhs = encrypt_bit_lvl0(l, &sk.key_tlwelvl0);
        let rhs = encrypt_bit_lvl0(r, &sk.key_tlwelvl0);
        let res = homand(lhs, rhs, &ck.key_bootstrap, &ck.key_keyswitch);
        let ans = decrypt_lvl0(&res.a, res.b, &sk.key_tlwelvl0.key_s);
        let end = start.elapsed();
        println!("{} AND {} = {}", l, r, ans);
        println!("{}{:03}ms", end.as_secs(), end.subsec_millis());
    }
}

pub fn xor_test() {
    //key_gen
    let sk: Secretkey = Secretkey::secretkey_gen();
    let ck: Cloudkey = Cloudkey::cloudkey_gen(&sk);

    for _i in 0..30 {
        let start = Instant::now();
        let mut rng = rand::thread_rng();
        let l = rng.gen_range(0..2);
        let r = rng.gen_range(0..2);
        let lhs = encrypt_bit_lvl0(l, &sk.key_tlwelvl0);
        let rhs = encrypt_bit_lvl0(r, &sk.key_tlwelvl0);
        let res = homxor(lhs, rhs, &ck.key_bootstrap, &ck.key_keyswitch);
        let ans = decrypt_lvl0(&res.a, res.b, &sk.key_tlwelvl0.key_s);
        let end = start.elapsed();
        println!("{} XOR {} = {}", l, r, ans);
        println!("{}{:03}ms", end.as_secs(), end.subsec_millis());
    }
}

#[allow(dead_code)]
pub fn fft_testtest() {
    let mut s: [u32; 512] = [0; 512];
    let mut a: [u32; 512] = [1; 512];
    let mut rng = rand::thread_rng();
    for i in s.iter_mut() {
        *i = rng.gen::<u32>();
    }
    for i in a.iter_mut() {
        *i = rng.gen_range(0..512);
    }
    let a_s = convolution_mod(&a, &s);
    let a_stest = fft_test(&a, &s);
    for i in 0..512 {
        if a_s[i] != a_stest[i] {
            println!("{}, {}", a_s[i], a_stest[i]);
        }
    }
}
