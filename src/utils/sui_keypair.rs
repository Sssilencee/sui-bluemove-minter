use base64::{
    engine::general_purpose, 
    Engine as _
};
use blake2::{
    Blake2b,
    Digest,
    digest::consts::U32
};
use ed25519_dalek::{
    Keypair,
    PublicKey,
    SecretKey,
    Signer,
};


type Blake2b256 = Blake2b<U32>;


pub trait SuiKeypair {
    // Create a keypair from a hexadecimal string
    fn from_hex(secret_hex: String) -> Self;

    // Sign the given bytes and return the signature as a Base64-encoded string
    fn sign_bs64(&self, tx_bytes: &[u8]) -> String;

    // Get the address associated with the keypair as a byte array
    fn get_address(&self) -> [u8; 32];
}

impl SuiKeypair for Keypair {
    fn from_hex(secret_hex: String) -> Self {
        // Decode the secret key from hexadecimal string
        let secret_bytes = hex::decode(secret_hex).unwrap();
        let secret_key = SecretKey::from_bytes(&secret_bytes[..]).unwrap();

        // Derive the public key from the secret key
        let public_key: PublicKey = (&secret_key).into();

        // Create a Keypair from the secret key and public key bytes
        Keypair::from_bytes(&[
            &secret_bytes[..],
            public_key.as_bytes()
        ].concat()).unwrap()
    }

    fn sign_bs64(&self, tx_bytes: &[u8]) -> String {
        // Intent signing, more: https://docs.sui.io/testnet/learn/cryptography/sui-intent-signing#user-signature
        let mut hasher = Blake2b256::new();
        hasher.update([&[0, 0, 0], tx_bytes].concat());

        // Compute the signature and concatenate the signature and public key
        let signature = [
            &self.sign(&hasher.finalize()[..]).to_bytes(), 
            &self.public.to_bytes()[..]
        ].concat();

        // Prepend a zero byte to indicate Ed25519 signature and encode as Base64
        let ed25519_signature = [&[0],  &signature[..]].concat();
        general_purpose::STANDARD.encode(&ed25519_signature)
    }

    fn get_address(&self) -> [u8; 32] {
        let mut hasher = Blake2b256::new();
        hasher.update([&[0x00], &self.public.to_bytes()[..]].concat());
        hasher.finalize().into()
    }
}