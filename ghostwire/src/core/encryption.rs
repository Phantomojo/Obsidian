use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::sealedbox;
use sodiumoxide::crypto::sign;
use sodiumoxide::init;
use anyhow::{Result, anyhow};
use rand::rngs::OsRng;

pub struct Encryption {
    secret_key: secretbox::Key,
    public_key: sign::PublicKey,
    secret_sign_key: sign::SecretKey,
}

impl Encryption {
    pub fn new() -> Result<Self> {
        init().map_err(|_| anyhow!("Failed to initialize sodiumoxide"))?;

        // Generate keypair for signing
        let (pk, sk) = sign::gen_keypair();

        // Generate a random secret key for symmetric encryption
        let secret_key = secretbox::gen_key();

        Ok(Self {
            secret_key,
            public_key: pk,
            secret_sign_key: sk,
        })
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        let nonce = secretbox::gen_nonce();
        let ciphertext = secretbox::seal(plaintext, &nonce, &self.secret_key);

        // Prepend nonce to ciphertext for transmission
        [nonce.0.to_vec(), ciphertext].concat()
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() < secretbox::NONCEBYTES {
            return Err(anyhow!("Ciphertext too short"));
        }
        let (nonce_bytes, ciphertext) = ciphertext.split_at(secretbox::NONCEBYTES);
        let nonce = secretbox::Nonce::from_slice(nonce_bytes).ok_or_else(|| anyhow!("Invalid nonce"))?;

        secretbox::open(ciphertext, &nonce, &self.secret_key).map_err(|_| anyhow!("Decryption failed"))
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        sign::sign(message, &self.secret_sign_key)
    }

    pub fn verify(&self, signed_message: &[u8], public_key: &sign::PublicKey) -> Result<Vec<u8>> {
        sign::verify(signed_message, public_key).map_err(|_| anyhow!("Signature verification failed"))
    }

    pub fn public_key(&self) -> &sign::PublicKey {
        &self.public_key
    }
}
