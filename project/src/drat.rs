use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

use aead::{Aead, KeyInit, Payload};
use aes_gcm_siv::{Aes256GcmSiv, Nonce};
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use x25519_dalek::{PublicKey, ReusableSecret};

type HmacSha256 = Hmac<Sha256>;

#[derive(Default, PartialEq, Eq, Clone)]
pub struct SymmKey([u8; 32]);

pub struct State {
    dhsk_snd: ReusableSecret,
    dhpk_snd: PublicKey,
    dhpk_rcv: PublicKey,
    rt_k: SymmKey,
    ck_snd: SymmKey,
    ck_rcv: SymmKey,
    n_snd: u64,
    n_rcv: u64,
    prev_n: u64,
    mk_skipped: HashMap<(PublicKey, u64), SymmKey>,
}

pub struct Header {
    dhpk: PublicKey,
    prev_n: u64,
    n: u64,
}

impl SymmKey {
    pub fn new(key_bytes: [u8; 32]) -> Self {
        SymmKey(key_bytes)
    }

    pub fn first_key_byte(&self) -> u8 {
        self.0[0]
    }
}

impl Display for SymmKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Symmkey({:?})", self.0)
    }
}

impl Header {
    pub const HEADER_LEN: usize = 48;
    pub const PK_LEN: usize = 32;

    fn new(dhpk: PublicKey, prev_n: u64, n: u64) -> Self {
        Self { dhpk, prev_n, n }
    }

    fn as_bytes(&self) -> [u8; Self::HEADER_LEN] {
        let mut bytes = [0u8; Self::HEADER_LEN];
        let (dhpk_dest, tail) = bytes.split_at_mut(Self::PK_LEN);
        let (prev_n_dest, n_dest) = tail.split_at_mut(8);
        dhpk_dest.copy_from_slice(self.dhpk.as_bytes());
        prev_n_dest.copy_from_slice(&self.prev_n.to_le_bytes());
        n_dest.copy_from_slice(&self.n.to_le_bytes());
        bytes
    }
}

impl State {
    pub const MAX_SKIP: u64 = 100;
    // helper functions

    /// function to derive new root key and chain key from shared secret and old root key using HKDF
    fn kdf_root(
        rt_k: &SymmKey,
        dhsk_snd: &ReusableSecret,
        dhpk_rcv: &PublicKey,
    ) -> (SymmKey, SymmKey) {
        let dh_out = dhsk_snd.diffie_hellman(dhpk_rcv);
        let hk = Hkdf::<Sha256>::new(Some(&rt_k.0), dh_out.as_bytes());
        let mut rk_bytes = [0u8; 32];
        let mut ck_bytes = [0u8; 32];
        hk.expand("root-keygen".as_bytes(), &mut rk_bytes).unwrap();
        hk.expand("chain-keygen".as_bytes(), &mut ck_bytes).unwrap();
        (SymmKey(rk_bytes), SymmKey(ck_bytes))
    }

    /// function to hash chain key using HMAC to return new chain key and message key
    ///
    /// ```
    /// use project::drat::{State, SymmKey};
    ///
    /// let k = SymmKey::new([0u8; 32]);
    /// let (ck, mk) = State::kdf_chain(&k);
    /// println!("{} {}", ck, mk);
    /// assert_eq!(ck.first_key_byte(), 92u8);
    /// assert_eq!(mk.first_key_byte(), 235u8);
    /// ```
    pub fn kdf_chain(chn_k: &SymmKey) -> (SymmKey, SymmKey) {
        let mut mac = <HmacSha256 as Mac>::new_from_slice(&chn_k.0).unwrap();
        mac.update(b"chain");
        let ck_bytes = mac.finalize_reset().into_bytes();
        mac.update(b"message");
        let mk_bytes = mac.finalize().into_bytes();
        (SymmKey(ck_bytes.into()), SymmKey(mk_bytes.into()))
    }

    /// function to encrypt message with key and associated data using AES256-GCM-SIV
    ///
    /// ```
    /// use project::drat::{State, SymmKey};
    ///
    /// let k = SymmKey::new([1u8; 32]);
    /// let msg = b"Hello, World!";
    /// let aad = b"Nothing important to add";
    /// let ctxt = State::aesgcmsiv_encrypt(&k, msg, aad);
    /// assert_eq!(ctxt[0], 152u8);
    ///
    /// ```
    pub fn aesgcmsiv_encrypt(key: &SymmKey, msg: &[u8], aad: &[u8]) -> Vec<u8> {
        let cipher = Aes256GcmSiv::new(&key.0.into());
        let nonce = Nonce::from_slice(b"Fixed nonce!".as_ref());
        cipher.encrypt(nonce, Payload { msg, aad }).unwrap()
    }

    /// function to decrypt ciphertext with key and associated data using AES256-GCM-SIV
    ///
    /// ```
    /// use project::drat::{State, SymmKey};
    ///
    /// let k = SymmKey::new([1u8; 32]);
    /// let msg = b"Hello, World!";
    /// let aad = b"Nothing important to add";
    /// let ctxt = State::aesgcmsiv_encrypt(&k, msg, aad);
    /// assert_eq!(State::aesgcmsiv_decrypt(&k, &ctxt, aad), msg);
    ///
    /// ```
    pub fn aesgcmsiv_decrypt(key: &SymmKey, msg: &[u8], aad: &[u8]) -> Vec<u8> {
        let cipher = Aes256GcmSiv::new(&key.0.into());
        let nonce = Nonce::from_slice(b"Fixed nonce!".as_ref());
        cipher.decrypt(nonce, Payload { msg, aad }).unwrap()
    }

    fn concat(header: &Header, aad: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Header::HEADER_LEN + aad.len());
        bytes.extend_from_slice(&header.as_bytes());
        bytes.extend_from_slice(aad);
        bytes
    }

    pub fn dh_keygen() -> (ReusableSecret, PublicKey) {
        let dhsk = ReusableSecret::random();
        let dhpk = PublicKey::from(&dhsk);
        (dhsk, dhpk)
    }

    // constructors
    pub fn ratchet_init_alice(shr_k: &SymmKey, bob_dhpk: PublicKey) -> Self {
        let (dhsk_snd, dhpk_snd) = Self::dh_keygen();
        let dhpk_rcv = bob_dhpk;
        let (rt_k, ck_snd) = Self::kdf_root(shr_k, &dhsk_snd, &dhpk_rcv);
        let ck_rcv = SymmKey::default();

        let n_snd = 0;
        let n_rcv = 0;
        let prev_n = 0;
        let mk_skipped = HashMap::new();

        State {
            dhpk_snd,
            dhsk_snd,
            dhpk_rcv,
            rt_k,
            ck_snd,
            ck_rcv,
            n_snd,
            n_rcv,
            prev_n,
            mk_skipped,
        }
    }

    pub fn ratchet_init_bob(
        shr_k: &SymmKey,
        bob_dhsk: ReusableSecret,
        bob_dhpk: PublicKey,
    ) -> Self {
        let dhsk_snd = bob_dhsk;
        let dhpk_snd = bob_dhpk;
        let dhpk_rcv = PublicKey::from([0; 32]);
        let rt_k = shr_k.clone();
        let ck_snd = SymmKey::default();
        let ck_rcv = SymmKey::default();

        let n_snd = 0;
        let n_rcv = 0;
        let prev_n = 0;
        let mk_skipped = HashMap::new();

        State {
            dhpk_snd,
            dhsk_snd,
            dhpk_rcv,
            rt_k,
            ck_snd,
            ck_rcv,
            n_snd,
            n_rcv,
            prev_n,
            mk_skipped,
        }
    }

    // ratchet functions
    pub fn ratchet_encrypt(&mut self, plaintext: &[u8], aad: &[u8]) -> (Header, Vec<u8>) {
        let msg_k: SymmKey;
        (self.ck_snd, msg_k) = Self::kdf_chain(&self.ck_snd);
        let header = Header::new(self.dhpk_snd, self.prev_n, self.n_snd);

        self.n_snd += 1;
        let full_aad = Self::concat(&header, aad);
        let ciphertext = Self::aesgcmsiv_encrypt(&msg_k, plaintext, &full_aad);
        (header, ciphertext)
    }

    pub fn ratchet_decrypt(&mut self, header: &Header, ciphertext: &[u8], aad: &[u8]) -> Vec<u8> {
        match self.try_skipped_message_keys(header, ciphertext, aad) {
            Some(plaintext) => plaintext,
            None => {
                if header.dhpk != self.dhpk_rcv {
                    self.skip_message_keys(header.prev_n);
                    self.dh_ratchet(header);
                }
                self.skip_message_keys(header.n);

                let msg_k: SymmKey;
                (self.ck_rcv, msg_k) = Self::kdf_chain(&self.ck_rcv);

                self.n_rcv += 1;
                let full_aad = Self::concat(header, aad);
                Self::aesgcmsiv_decrypt(&msg_k, ciphertext, &full_aad)
            }
        }
    }

    fn try_skipped_message_keys(
        &mut self,
        header: &Header,
        ciphertext: &[u8],
        aad: &[u8],
    ) -> Option<Vec<u8>> {
        match self.mk_skipped.remove(&(header.dhpk, header.n)) {
            Some(msg_k) => {
                let full_aad = Self::concat(header, aad);
                Some(Self::aesgcmsiv_decrypt(&msg_k, ciphertext, &full_aad))
            }
            None => None,
        }
    }

    fn skip_message_keys(&mut self, until: u64) {
        if self.n_rcv + Self::MAX_SKIP < until {
            panic!();
        }
        if self.ck_rcv != SymmKey([0u8; 32]) {
            while self.n_rcv < until {
                let msg_k: SymmKey;
                (self.ck_rcv, msg_k) = Self::kdf_chain(&self.ck_rcv);
                self.mk_skipped.insert((self.dhpk_rcv, self.n_rcv), msg_k);
                self.n_rcv += 1;
            }
        }
    }

    fn dh_ratchet(&mut self, header: &Header) {
        self.prev_n = self.n_snd;
        self.n_snd = 0;
        self.n_rcv = 0;
        self.dhpk_rcv = header.dhpk;
        (self.rt_k, self.ck_rcv) = Self::kdf_root(&self.rt_k, &self.dhsk_snd, &self.dhpk_rcv);

        (self.dhsk_snd, self.dhpk_snd) = Self::dh_keygen();
        (self.rt_k, self.ck_snd) = Self::kdf_root(&self.rt_k, &self.dhsk_snd, &self.dhpk_rcv);
    }
}
