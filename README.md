<p align="center">
  <a href="https://gitpod.io/#https://github.com/EugenWay/VottingApp">
    <img src="https://gitpod.io/button/open-in-gitpod.svg" width="240" alt="GEAR">
  </a>
</p>

# Gear Voting App

The first smart contract on Rust tutorial

## Prebuilt Binaries

➡️ https://github.com/EugenWay/VottingApp/releases/tag/build

- Output WASM: [voting_app.wasm](https://github.com/EugenWay/VottingApp/releases/download/build/voting_app.wasm)
- Optimized WASM: [voting_app.opt.wasm](https://github.com/EugenWay/VottingApp/releases/download/build/gear_feeds_channel.opt.wasm)
- Meta WASM: [voting_app.meta.wasm](https://github.com/EugenWay/VottingApp/download/build/gear_feeds_channel.meta.wasm)

## Building Locally

### ⚙️ Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ⚒️ Add specific toolchains

```shell
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install --git https://github.com/gear-tech/gear wasm-proc
```

### 🏗️ Build

```shell
RUSTFLAGS="-C link-args=--import-memory" cargo +nightly build --release --target=wasm32-unknown-unknown
wasm-proc --path ./target/wasm32-unknown-unknown/release/voting_app.wasm
```

## Using

### 📦 Install Polkadot.js Extension

Download and install Polkadot.js browser extension: https://polkadot.js.org/extension/

### 👛 Create Account

Create a new account using Polkadot.js extension. **Don't forget to save the mnemonic seed phrase and password in a safe place.**

### ✉️ Upload the Program

- Go to https://idea.gear-tech.io/
- Connect to your account using the **Connect** button. Allow website access to your wallet in Polkadot.js extension.
- Top up yout test account using the **Get test account** button. This button can be pressed several times.
- Upload the program (`.opt.wasm`) and metadata (`.meta.wasm`) giving some meaninful name to the program and setting the gas limit to `100'000'000`. Sign the transaction using Polkadot.js extension.
- Find the program in **Recently uploaded programs** section and copy its address.

### 📒 Add new Candidate/Vote for Candidate

- Find your program in the **All programs** section and open the message sending form.
- Add new candidate or vote for existing one.
- Set the **Gas limit** to `300'000'000` and click **Send request**. Sign the transaction using Polkadot.js extension.

### 📒 Read State

- In program page go to **Read state**
- Provide candidate name as String to get the number of votes for it, or let input empty to receive all existing candidates.

## License

The source code is licensed under [GPL v3.0 license](LICENSE).