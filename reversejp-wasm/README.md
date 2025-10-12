# reversejp-wasm

WebAssembly bindings for the [`reversejp`](../reversejp-rust) reverse-geocoding engine.

## Build

```bash
make build
```

The command above produces a `pkg/` directory containing the WebAssembly module and TypeScript bindings that can be published to npm or consumed directly.

## Usage

```js
import initWasm, { initialize, find_properties } from "reversejp-wasm";

await initWasm();
initialize();

const results = find_properties(139.7670, 35.6812);
console.log(results);
```

`find_properties` returns an array of objects with the same shape as the `Properties` struct in the Rust crate: `{ code, name, enName }`.
