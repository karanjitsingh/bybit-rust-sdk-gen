# Bybit Rust SDK - Transpiler Issues Tracker

## Status: ✅ ALL COMPILATION ERRORS RESOLVED

The SDK compiles successfully with 0 errors and 63 warnings (all benign: unused
variables from stub methods, expected for generated code).

---

## Hand-Written Files

Hand-written files live in `hand-written/` and are copied into `bybit-rust-sdk/` after
`npm run gen` runs the transpiler. This keeps them out of the generated directory so
regeneration is always clean.

```
hand-written/
  Cargo.toml
  Cargo.lock
  src/client/
    BaseRestClient.rs
    BaseWebsocketClient.rs
    config.rs
    signing.rs
```

The `gen` script in `package.json` runs: `node bin/parser.js && cp -r hand-written/...`

---

## Issue 1: Transpiler crashes on `Buffer` type (FATAL)
- **File**: `src/typeConverter.ts` line 11
- **Fix**: Added `"Buffer"` to `EXTERNAL_TYPES` array
- **Status**: [x] DONE

## Issue 2: Skipped generic type alias `InstrumentInfoResponseV5`
- **File**: `src/bybitClientHandlers.ts` (`convertTypeToRust`)
- **Fix**: When a generic base type isn't in the registry, fall back to `serde_json::Value`
- **Status**: [x] DONE

## Issue 3: `CategoryListV5` generic arity mismatch
- **File**: `src/bybitClientHandlers.ts` (`convertTypeToRust`)
- **Fix**: When a known type gets 3+ generic args (from TS overload merging), fall back to `serde_json::Value`
- **Status**: [x] DONE

## Issue 4: `void` return type mapped incorrectly
- **File**: `src/bybitClientHandlers.ts` (`generateRustMethod`)
- **Fix**: Added `.map(|_| ())` when return type is `()` to discard the response value
- **Status**: [x] DONE

## Issue 5: `EmittableEvent_EventType` not generated
- **File**: `src/parser.ts`
- **Fix**: Moved util folder parsing BEFORE inline type collection so inline types from
  util interfaces are included. Also fixed `::mod` in inline type module paths.
- **Status**: [x] DONE

## Issue 6: 20+ WS event type aliases skipped (generic instantiations)
- **File**: `src/codeGenerator.ts` (`convertTypeAlias`)
- **Fix**: Added detection of generic instantiation aliases (`type X = Y<A, B>`) and
  emit `pub type X = Y<A', B'>` with converted args
- **Status**: [x] DONE

## Issue 7: Array parameter types not resolved in client methods
- **File**: `src/bybitClientHandlers.ts` (validation in `generateClient`)
- **Fix**: Strip `[]`, `<...>`, and `& ...` from type names before registry lookup
- **Status**: [x] DONE

## Issue 8: Intersection type params not resolved
- **File**: `src/bybitClientHandlers.ts` (validation in `generateClient`)
- **Fix**: Same as Issue 7 — strip `& ...` before registry lookup
- **Status**: [x] DONE

## Issue 9: 3 skipped type-level map interfaces (WS API)
- **Status**: [x] WONTFIX (acceptable — TS-only type safety constructs)

## Issue 10: `DeferredPromiseRef` typeof/keyof resolution failure
- **Status**: [x] WONTFIX (acceptable — falls back to String)

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
- Compilation: 0 errors, 63 warnings (all benign)
- Skipped types: 7 (complex TS-only constructs)
