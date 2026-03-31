use k256::ecdsa::SigningKey;
use k256::elliptic_curve::rand_core::OsRng;
use tiny_keccak::{Hasher, Keccak};

pub struct EthWallet {
    pub private_key: String,
    pub public_key: String,
    pub address: String,
}

pub fn generate_wallet() -> EthWallet {
    let signing_key = SigningKey::random(&mut OsRng);

    let private_bytes = signing_key.to_bytes();
    let private_key = format!("0x{}", hex::encode(private_bytes));

    let verifying_key = signing_key.verifying_key();
    let public_point = verifying_key.to_encoded_point(false); // uncompressed
    let public_bytes = public_point.as_bytes();

    let public_key = format!("0x{}", hex::encode(public_bytes));

    // Ethereum address = keccak256(pubkey[1..]) last 20 bytes
    let pubkey_no_prefix = &public_bytes[1..];

    let mut keccak = Keccak::v256();
    let mut output = [0u8; 32];
    keccak.update(pubkey_no_prefix);
    keccak.finalize(&mut output);

    let address_bytes = &output[12..];
    let address = format!("0x{}", hex::encode(address_bytes));

    EthWallet {
        private_key,
        public_key,
        address,
    }
}
