use getrandom::getrandom;
use ring::aead::{self, BoundKey};
use std::fs::File;
use std::io::{Read, Write};

struct MyNonce([u8; aead::NONCE_LEN]);

impl aead::NonceSequence for MyNonce {
    fn advance(&mut self) -> Result<aead::Nonce, ring::error::Unspecified> {
        Ok(aead::Nonce::assume_unique_for_key(self.0))
    }
}

pub fn encrypt(input_path: &str, output_path: &str, key_bytes: &[u8; 16]) -> std::io::Result<()> {
    let mut in_file = File::open(input_path)?;
    let mut contents: Vec<u8> = vec![];
    in_file.read_to_end(&mut contents)?;

    let mut nonce_seed = [0u8; aead::NONCE_LEN];
    getrandom(&mut nonce_seed).unwrap();
    let nonce_sequence = MyNonce(nonce_seed);

    let key = aead::UnboundKey::new(&aead::AES_128_GCM, key_bytes).unwrap();
    let mut enc_key = aead::SealingKey::new(key, nonce_sequence);

    let mut out_file = File::options()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)?;

    let tag = enc_key
        .seal_in_place_separate_tag(aead::Aad::empty(), &mut contents)
        .unwrap();
    out_file.write_all(&nonce_seed)?;
    out_file.write_all(tag.as_ref())?;
    out_file.write_all(&contents)?;

    Ok(())
}

pub fn decrypt(input_path: &str, output_path: &str, key_bytes: &[u8; 16]) -> std::io::Result<()> {
    let mut in_file = File::open(input_path)?;
    let mut contents: Vec<u8> = vec![];

    let mut nonce_seed = [0u8; aead::NONCE_LEN];
    in_file.read_exact(&mut nonce_seed)?;
    let nonce_sequence = MyNonce(nonce_seed);

    let mut tag_bytes = [0u8; aead::MAX_TAG_LEN];
    in_file.read_exact(&mut tag_bytes)?;

    in_file.read_to_end(&mut contents)?;
    contents.extend_from_slice(&tag_bytes);

    let key = aead::UnboundKey::new(&aead::AES_128_GCM, key_bytes).unwrap();
    let mut dec_key = aead::OpeningKey::new(key, nonce_sequence);

    let mut out_file = File::options()
        .read(false)
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)?;

    let ct = dec_key
        .open_in_place(aead::Aad::empty(), &mut contents)
        .unwrap();

    out_file.write_all(ct)?;

    Ok(())
}
