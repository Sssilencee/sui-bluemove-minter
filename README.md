# sui-bluemove-minter

The `sui-bluemove-minter` is a minter designed for the [Bluemove launchpad](https://sui.bluemove.net/launchpad) that does not rely on the [sui-sdk](https://github.com/MystenLabs/sui/tree/main/crates/sui-sdk).

## Dependencies

The project has the following dependencies:

- `anyhow` (version 1.0.70)
- `serde` (version 1.0.160)
- `bcs` (version 0.1.5) for Binary Canonical Serialization of the TransactionData structure
- `tokio` (version 1.28.0) with features `rt-multi-thread` and `macros` for async main function support
- `hex` (version 0.4.3) for hexadecimal encoding and decoding of Sui addresses
- `serde_json` (version 1.0.96) for serialization and deserialization of structures for the JSON RPC API
- `blake2` (version 0.10.6) for Blake2b hashing used for signature and address derivation
- `bs58` (version 0.4.0) for decoding gas object digests
- `ed25519-dalek` (version 1.0.1) for ECDSA signing and public key derivation
- `base64` (version 0.21.0) for encoding bytes of signatures and TransactionData for the JSON RPC API
- `reqwest` (version 0.11.17) for making HTTP requests to the JSON RPC API

## Setup

To set up the project, follow these steps:

1. Create a `config.toml` file and add your data to it.
2. In the `config.toml` file, specify the following parameters:
   - `price`: The price of the item in MIST.
   - `count`: The number of NFTs you want to mint.
   - `sale_type`: The type of sale, for example: OG/Whitelist/Public - Og=0;Whitelist=1;Public=2 or Whitelist/Public - Whitelist=0;Public=1, etc.
   - `secret_key`: Your wallet's hexadecimal secret key without the `0x` prefix.
   - `cap_address` and `mint_address`: These addresses can be obtained from the following request: `https://sui-api-mainnet.bluemove.net/api/launchpads...`
   - `gas_address`: Your wallet's owned Sui account address.

Once you have set up the configuration file, you can proceed with the following steps:

1. Build the project using the command: `cargo build`.
2. Run the project using the created configuration file: `cargo run --config config.toml`.

## Project Structure

The project is structured as follows:

- **Types**
  - `http`: This module contains the types for JSON RPC API calls and responses.
  - `sui`: This module includes the Sui types for the TransactionData structure.

- **Utils**
  - `rpc_client`: This module provides an implementation of the `reqwest::Client` for interacting with the JSON RPC API.
  - `sui_keypair`: This module defines a trait that implements the `ed25519_dalek` crate for offline signing.

The project structure separates different functionality into modules, making it organized and easier to maintain. The `Types` module contains types related to the JSON RPC API and Sui-specific types. The `Utils` module provides utility functions for interacting with the API and handling key pairs for offline signing using the `ed25519_dalek` crate.
