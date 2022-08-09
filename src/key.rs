use crate::{
    sampleextractindex::sample_extract_index_key,
    tlwe::{TlweKeylvl0, TlweKeylvl1},
    trgsw::{bootstrappingkey_gen, keyswtching_gen, KeySwitch, Trgsw},
    trlwe::TrlweKey,
};

pub struct Secretkey {
    pub key_tlwelvl0: TlweKeylvl0,
    pub key_tlwelvl1: TlweKeylvl1,
    pub key_trlwe: TrlweKey,
}

impl Secretkey {
    pub fn secretkey_gen() -> Self {
        let sk_tlwe0 = TlweKeylvl0::keygen();
        let sk_trlwe = TrlweKey::keygen();
        let sk_tlwe1 = sample_extract_index_key(&sk_trlwe);
        Self {
            key_tlwelvl0: (sk_tlwe0),
            key_tlwelvl1: (sk_tlwe1),
            key_trlwe: (sk_trlwe),
        }
    }
}

pub struct Cloudkey {
    pub key_bootstrap: Vec<Trgsw>,
    pub key_keyswitch: KeySwitch,
}

impl Cloudkey {
    pub fn cloudkey_gen(sk: &Secretkey) -> Self {
        let ck_bootstrap = bootstrappingkey_gen(&sk.key_tlwelvl0, &sk.key_trlwe);
        let ck_keyswitch = keyswtching_gen(&sk.key_tlwelvl1, &sk.key_tlwelvl0);
        Self {
            key_bootstrap: (ck_bootstrap),
            key_keyswitch: (ck_keyswitch),
        }
    }
}
