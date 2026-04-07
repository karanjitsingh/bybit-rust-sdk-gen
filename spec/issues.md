# Bybit Rust SDK - Transpiler Issues Tracker

## Status: ✅ Compiles — 0 errors, 83 warnings (all benign)

`bybit-api` submodule pinned at `68ade13` (pre-#536). Latest master requires Issue 13.

---

## Hand-Written Files

Hand-written files live in `hand-written/` and are copied into `bybit-rust-sdk/` after
`npm run gen` runs the transpiler. This keeps them out of the generated directory so
regeneration is always clean.

```
hand-written/
  Cargo.toml
  Cargo.lock
  rustfmt.toml
  src/client/
    BaseRestClient.rs
    BaseWebsocketClient.rs
    config.rs
    signing.rs
```

The `gen` script in `package.json` runs: `node bin/parser.js && cp -r hand-written/...`

---

## Design Constraint: No Struct Duplication

The upstream TS SDK reuses interfaces heavily across request/response/websocket types.
The transpiler preserves this — shared types stay in `shared.rs` / `shared_v5.rs`,
and the `TypeRegistry` + `InlineTypeRegistry` ensure each type is registered once by
signature. Fixes use the existing registries and never emit duplicate structs.

---

## Build Stats
- Total generated types: 814
- Total files: 55
- Compilation: 0 errors, 83 warnings (all benign)
- Skipped types: 7 (complex TS-only constructs)
- Remaining stubs: 29 (todo!/unimplemented!) — need hand-written Rust implementations

---

## Done

### Issue 1: Transpiler crashes on `Buffer` type (FATAL)
- **File**: `src/typeConverter.ts` line 11
- **Fix**: Added `"Buffer"` to `EXTERNAL_TYPES` array

### Issue 2: Skipped generic type alias `InstrumentInfoResponseV5`
- **File**: `src/bybitClientHandlers.ts` (`convertTypeToRust`)
- **Fix**: When a generic base type isn't in the registry, fall back to `serde_json::Value`

### Issue 3: `CategoryListV5` generic arity mismatch
- **File**: `src/bybitClientHandlers.ts` (`convertTypeToRust`)
- **Fix**: When a known type gets 3+ generic args (from TS overload merging), fall back to `serde_json::Value`

### Issue 4: `void` return type mapped incorrectly
- **File**: `src/bybitClientHandlers.ts` (`generateRustMethod`)
- **Fix**: Added `.map(|_| ())` when return type is `()` to discard the response value

### Issue 5: `EmittableEvent_EventType` not generated
- **File**: `src/parser.ts`
- **Fix**: Moved util folder parsing BEFORE inline type collection so inline types from
  util interfaces are included. Also fixed `::mod` in inline type module paths.

### Issue 6: 20+ WS event type aliases skipped (generic instantiations)
- **File**: `src/codeGenerator.ts` (`convertTypeAlias`)
- **Fix**: Added detection of generic instantiation aliases (`type X = Y<A, B>`) and
  emit `pub type X = Y<A', B'>` with converted args

### Issue 7: Array parameter types not resolved in client methods
- **File**: `src/bybitClientHandlers.ts` (validation in `generateClient`)
- **Fix**: Strip `[]`, `<...>`, and `& ...` from type names before registry lookup

### Issue 8: Intersection type params not resolved
- **File**: `src/bybitClientHandlers.ts` (validation in `generateClient`)
- **Fix**: Same as Issue 7 — strip `& ...` before registry lookup

### Issue 9: 3 skipped type-level map interfaces (WS API)
- WONTFIX (acceptable — TS-only type safety constructs)

### Issue 10: `DeferredPromiseRef` typeof/keyof resolution failure
- WONTFIX (acceptable — falls back to String)

### Issue 11: WebSocket API methods generated as `todo!()` stubs
- **Root cause**: WS regex didn't match `this.wsClient.sendWSAPIRequest(WS_KEY_MAP.xxx, 'op', params)` — the regex expected simple variable for params but batch methods pass object literals `{ category, request: orders }`
- **Fix**: Updated WS regex to allow object literal params. Fixed `operationToEnumVariant` to split on `-` (e.g. `order.create-batch` → `OrderCreateBatch`). Fixed body generation to serialize params with `serde_json::to_value()` and build `serde_json::json!({...})` for multi-param methods.
- **Result**: 6 WS API methods now generate real implementations (`submitNewOrder`, `amendOrder`, `cancelOrder`, `batchSubmitOrders`, `batchAmendOrder`, `batchCancelOrder`)

---

## Remaining

### Issue 12: 29 `todo!()`/`unimplemented!()` stubs need hand-written Rust
These methods have complex logic that can't be auto-transpiled. Hand-write in `hand-written/`.

| File | Method | Notes |
|------|--------|-------|
| WebsocketAPIClient.rs | `getWSClient` | Simple: return `&self.ws_client` |
| WebsocketAPIClient.rs | `setTimeOffsetMs` | Simple: delegate to ws_client |
| RestClientV5.rs | `getClientType` | Simple: return constant |
| RestClientV5.rs | `fetchServerTime` | Call `get_server_time()` + math |
| RestClientV5.rs | `fetchLatencySummary` | Multi-step latency calculation |
| RestClientV5.rs | `uploadP2PChatFile` | Needs `post_private_file` on BaseRestClient |
| SpotClientV3.rs | `getClientType` | Simple: return constant |
| SpotClientV3.rs | `fetchServerTime` | Call `get_server_time()` + conversion |
| WebsocketClient.rs | 4 abstract methods | `unimplemented!()` — correct, override points |
| WebsocketClient.rs | `connectWSAPI` | Calls `assertIsAuthenticated` |
| WebsocketClient.rs | `connectPublic` | Switch on market, calls `connect` per ws key |
| WebsocketClient.rs | `connectPrivate` | Switch on market, calls `connect` |
| WebsocketClient.rs | `subscribeV5` | Topic routing + batch subscribe loop |
| WebsocketClient.rs | `unsubscribeV5` | Topic routing + batch unsubscribe loop |
| WebsocketClient.rs | `subscribe` | Normalise topics, route per ws key |
| WebsocketClient.rs | `unsubscribe` | Normalise topics, route per ws key |
| WebsocketClient.rs | `getWsUrl` | Build WS URL + auth suffix |
| WebsocketClient.rs | `sendPingEvent` | JSON `{ op: "ping" }` via `tryWsSend` |
| WebsocketClient.rs | `sendPongEvent` | JSON `{ op: "pong" }` via `tryWsSend` |
| WebsocketClient.rs | `getMaxTopicsPerSubscribeEvent` | Delegate to util function |
| WebsocketClient.rs | `getPrivateWSKeys` | Return `WS_AUTH_ON_CONNECT_KEYS` |
| WebsocketClient.rs | `isAuthOnConnectWsKey` | Check if key in auth list |
| WebsocketClient.rs | `isPrivateTopicRequest` | Check topic name against private list |
| WebsocketClient.rs | `isWsPing` | Parse message data for ping op |
| WebsocketClient.rs | `isWsPong` | Parse message data for pong op |
| WebsocketClient.rs | `resolveEmittableEvents` | JSON parse + event type routing |

### Issue 13: `bybit-api` master compatibility — new TS type patterns
- **Blocker**: Upstream master (`1589319`, PR #536+) added `Parameters<WSClientEventMap<string>[TEventType]>[0]` in `util/BaseWSClient.ts`
- **Error**: `Unknown type 'Parameters<WSClientEventMap<string>[TEventType]>[0]'. Type not found in registry.`
- **Fix needed**: `src/typeConverter.ts` — handle `Parameters<...>[N]` indexed access types (fall back to `serde_json::Value`)
- **Impact**: `bybit-api` submodule pinned at `68ade13` until this is fixed
