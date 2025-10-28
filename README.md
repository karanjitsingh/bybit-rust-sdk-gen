# bybit-rust-sdk-gen

⚠️⚠️⚠️ **AI Slop Disclaimer** ⚠️⚠️⚠️ This code was heavily generated using AI, use it at your own risk. I will not be responsible for your financial losses.

--- 

`bybit-rust-sdk-gen` is a transpiler written for ByBit's NodeJs community SDK [tiagosiebler/bybit-api](https://github.com/tiagosiebler/bybit-api) converting the TS/JS sdk to Rust.



Build the typescript library first with `npm run build`. Running `npm run gen` will genearte the typescript Rust SDK into `./bybit-rust-sdk`. Note that the transpiler only converts interfaces and other types and not actual execution code, some bits of the sdk are still written by hand:
- `bybit-rust-sdk/`
   - `Cargo.toml`
   - `src/client/BaseRestClient.rs`
   - `src/client/BaseWebsocketClient.rs`
   - `src/client/config.rs`
   - `src/client/signing.rs`

```bash
# Build the typescript lib
npm run build

# Generate sdk
npm run gen
```