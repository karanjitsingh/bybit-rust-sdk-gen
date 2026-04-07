# Bybit Rust SDK - Transpiler Issues Tracker

## Status: ✅ Compiles — 0 errors, 57 warnings (all benign)

`bybit-api` submodule now on latest master (`1589319`, PR #536+).

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
- Total generated types: 891
- Total files: 56
- Compilation: 0 errors, 81 warnings (all benign)
- Skipped types: 7 (complex TS-only constructs)
- Remaining stubs: 17 (13 todo!/4 unimplemented!) — 15 implemented via knownImpls

---

## AI Context Notes

These notes help resume work across sessions. Read this section first.

### Architecture Overview
- `src/parser.ts` — Main orchestrator. Two-pass: (1) register all types, (2) generate Rust code.
  Types go into `FileStructure` objects keyed by Rust file path. Client files get special handling:
  interfaces/type aliases extracted via `BybitHandlers.extractClientTypes()` → added to `client/mod.rs`.
- `src/typeConverter.ts` — `TypeConverter.convert()` maps TS types → Rust types. Handles primitives,
  arrays, unions, objects, generics, inline objects. Falls back to `serde_json::Value` for complex
  TS-only constructs (Parameters<>, keyof, indexed access types).
- `src/codeGenerator.ts` — `convertInterface()`, `convertTypeAlias()`, `convertEnum()`. Generates
  Rust struct/enum/type alias code. Filters unused generic params from struct declarations.
- `src/bybitClientHandlers.ts` — `convertTypeToRust()` for client method signatures (separate from
  TypeConverter — works on string types, not ts-morph Type objects). `generateRustMethod()` generates
  method bodies. `generateClientImports()` resolves use statements.
- `src/fileStructure.ts` — `FileStructure` class. Holds types, inline types, imports per file.
  `write()` generates the final `.rs` file. Inline types go in a `pub mod inline {}` submodule.
- `src/fileGenerator.ts` — `generateModFile()` for mod.rs files. Client mod.rs has hardcoded
  infrastructure (ClientError, ClientResult). Extracted types appended after in parser.ts.
- `src/inlineTypeRegistry.ts` — Deduplicates inline union types by signature. Tracks which files
  use each inline type. Types used in 2+ files get promoted to a common file.
- `src/bybitHandlers.ts` — `extractClientTypes()` pulls interfaces/type aliases from client TS files
  into `client/mod.rs`. Registers them with module path `crate::client`.

### Key Patterns & Gotchas
- **Two type converters**: `TypeConverter.convert()` (ts-morph Type objects, used during interface
  processing) vs `convertTypeToRust()` in bybitClientHandlers (string-based, used for client method
  return types/params). Both need parallel fixes for new TS patterns.
- **Inline objects in generic interfaces**: If an inline object field references a parent's generic
  param (e.g., `result: { data: TResult }`), the inline struct can't declare that generic. The
  converter detects this and falls back to `serde_json::Value` for the whole field.
- **Unused generic params**: After inline object conversion, a parent struct may have unused generics.
  `convertInterface()` filters these out before emitting the struct declaration.
- **client/mod.rs generation order**: `fileStructure.write()` runs first (writes extracted types),
  then `generateModFile()` overwrites with the hardcoded template. Extracted types are re-appended
  in parser.ts after `generateModFile()`.
- **String literal variants in heterogeneous enums**: Lowercase variants (e.g., `connectionReady`)
  are string literals, not type names. They generate `VariantName(String)` not `name(name)`.
- **Response wrapper generic args**: TS response wrappers like `APIP2PResponse<T, U>` may have
  fewer Rust generic params after inline object conversion. `convertTypeToRust()` truncates
  excess args for types matching `/Response|API/`.

### Recent Fixes (Issue 13 + related)
Files modified: typeConverter.ts, bybitHandlers.ts, bybitClientHandlers.ts, codeGenerator.ts,
fileStructure.ts, fileGenerator.ts, parser.ts

1. **Parameters<...>[N] indexed access types** — regex in typeConverter.ts `convert()` before
   final throw. Falls back to `serde_json::Value`.
2. **keyof expressions** — handled in both typeConverter.ts and bybitClientHandlers.ts.
   Maps to `String` in typeConverter, `String` in convertTypeToRust.
3. **null type mapping** — changed from `()` to `Option<serde_json::Value>` for struct fields.
4. **() stripping bug** — `codeGenerator.ts` was stripping `()` unit type via parentheses removal.
   Added `!== '()'` guard.
5. **Extracted client types module path** — `bybitHandlers.ts` was registering with `client/mod.rs`
   instead of `crate::client`. Fixed to use `crate::client`.
6. **Hardcoded types in client/mod.rs** — Removed `DeferredPromiseRef` and
   `WSAPIClientConfigurableOptions` from hardcoded template since they come from extraction now.
7. **Extracted types appended after generateModFile** — parser.ts appends extracted types to
   client/mod.rs after the hardcoded template is written.
8. **requestFlags parameter** — upstream added optional `requestFlags: WSAPIRequestFlags` to
   `sendWSAPIRequest`. Added `None` to generated WS API calls.
9. **String literal variants in heterogeneous enums** — Fixed in fileStructure.ts and
   fileGenerator.ts. Lowercase variants now generate `VariantName(String)`.
10. **Default impl for generic heterogeneous enums** — Added manual `Default` impl in
    fileStructure.ts for enums like `EmittableEvent_EventType<T>`.
11. **Unused generic param filtering** — codeGenerator.ts `convertInterface()` now checks which
    generic params are actually used in field types and removes unused ones.
12. **Response wrapper generic arg truncation** — bybitClientHandlers.ts `convertTypeToRust()`
    truncates excess generic args for response wrapper types.
13. **Inline objects referencing parent generics** — typeConverter.ts detects when an inline
    object field references a parent's generic param and falls back to `serde_json::Value`.

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

### Issue 13: `bybit-api` master compatibility — new TS type patterns
- **Root cause**: Upstream master added `Parameters<WSClientEventMap<string>[TEventType]>[0]`,
  `keyof WSClientEventMap<string>`, `WSAPIRequestFlags` interface, and other new patterns.
- **Fix**: 13 sub-fixes across 7 files (see AI Context Notes → Recent Fixes above)
- **Result**: `bybit-api` submodule updated to latest master. 0 errors, 57 warnings, 891 types.

---

## Remaining

### Issue 12: 17 remaining `todo!()`/`unimplemented!()` stubs
15 simple methods implemented via `knownImpls` map in `bybitClientHandlers.ts`.
Added `try_ws_send` to `hand-written/src/client/BaseWebsocketClient.rs`.

Implemented: `getClientType` (×2), `fetchServerTime` (×2), `setTimeOffsetMs`,
`sendPingEvent`, `sendPongEvent`, `getPrivateWSKeys`, `isAuthOnConnectWsKey`,
`authPrivateConnectionsOnConnect`, `isCustomReconnectionNeeded`, `isWsPing`,
`isWsPong`, `getMaxTopicsPerSubscribeEvent`, `isPrivateTopicRequest`.

Remaining stubs (complex — need full WS/REST infrastructure):

| File | Method | Notes |
|------|--------|-------|
| WebsocketAPIClient.rs | `getWSClient` | Needs signature change (return ref) |
| RestClientV5.rs | `fetchLatencySummary` | Multi-step latency calculation |
| RestClientV5.rs | `uploadP2PChatFile` | Needs multipart upload on BaseRestClient |
| WebsocketClient.rs | 4 abstract methods | `unimplemented!()` — correct, override points |
| WebsocketClient.rs | `connectWSAPI` | Needs `assertIsAuthenticated` on base |
| WebsocketClient.rs | `connectPublic` | Needs `connect` per ws key on base |
| WebsocketClient.rs | `connectPrivate` | Needs `connect` on base |
| WebsocketClient.rs | `subscribeV5` | Topic routing + batch subscribe loop |
| WebsocketClient.rs | `unsubscribeV5` | Topic routing + batch unsubscribe loop |
| WebsocketClient.rs | `subscribe` | Normalise topics, route per ws key |
| WebsocketClient.rs | `unsubscribe` | Normalise topics, route per ws key |
| WebsocketClient.rs | `getWsUrl` | Build WS URL + auth suffix |
| WebsocketClient.rs | `triggerCustomReconnectionWorkflow` | Custom reconnection logic |
| WebsocketClient.rs | `resolveEmittableEvents` | JSON parse + event type routing |
