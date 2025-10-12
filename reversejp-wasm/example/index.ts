import initWasm, { initialize, find_properties } from "reversejp-wasm";

await initWasm();
initialize();

const results = find_properties(139.767, 35.6812);
console.log(results);
