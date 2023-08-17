# Ethereum Address Generator

## Description

This Rust application generates a new address for the Ethereum network. It creates a private-public key pair and then derives an Ethereum address from this pair. If the generated address starts with a specific prefix (e.g., "0x123456"), the address and the keys are saved into a file named `addresses.json`.

## Dependencies

- `anyhow`
- `secp256k1`
- `tiny_keccak`
- `web3`
- `serde_json`
- `tokio`
- `std`

## Installation

First, clone the project:

```bash
git clone https://github.com/codeesura/Ethereum-address-generator-rust.git
cd Ethereum-address-generator-rust
```

- Then, install the dependencies:

```bash
cargo build
```

## Usage

While in the project's main directory, run the application with:


## How It Works

1. **Key Pair Generation and Address Calculation:**
   - A new `secp256k1` object is instantiated with `Secp256k1::new()`.
   - A timestamp function is defined to create a Random Number Generator (RNG).
   - A key pair (`secret_key` and `pub_key`) is generated using this RNG.
   - The `public_key_address` function derives an Ethereum address from the given `PublicKey` object.

2. **Filtering Part:**
   - The generated Ethereum address is checked to see if it starts with a specific prefix (e.g., "0x123456").
   - If this condition is met, the address and the associated keys are converted into a JSON object.

3. **Reading and Writing to File:**
   - The `addresses.json` file is read, and the existing data is fetched.
   - The newly created JSON object is appended to this data.
   - The updated data is written back into the `addresses.json` file.

## Contributing

If you would like to contribute to the project, please open an issue first. Pull Requests (PRs) are always welcome.

## License

This project is licensed under the [MIT License](https://github.com/codeesura/Ethereum-address-generator-rust/blob/main/LICENSE).



