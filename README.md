# bybit-rust-sdk-gen

⚠️⚠️⚠️ **AI Slop Disclaimer** ⚠️⚠️⚠️ Most of this code was generated using AI and was written as a personal project. Use it at your own risk, I will not be responsible for your financial losses.

--- 

`bybit-rust-sdk-gen` is a transpiler written for ByBit's NodeJs community SDK [tiagosiebler/bybit-api](https://github.com/tiagosiebler/bybit-api) converting the TS/JS sdk to Rust.

This is currently a work in progress, the generated code does not compile.

1. Initialize submodules (`bybit-api` and `bybit-rust-sdk`):
   ```
   git submodule update --init --recursive
   ```

1. Build the typescript transpiler with `npm run build`. Then run `npm run gen` to generate the Rust SDK into `./bybit-rust-sdk`. Note that the transpiler only converts interfaces and other types and not actual execution code, some bits of the sdk are still written by hand:
   - `bybit-rust-sdk/`
      - `Cargo.toml`
      - `src/client/BaseRestClient.rs`
      - `src/client/BaseWebsocketClient.rs`
      - `src/client/config.rs`
      - `src/client/signing.rs`
