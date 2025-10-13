import init, { initialize, find_properties } from "reversejp-wasm";

await init();
initialize();

const results = find_properties(139.767, 35.6812);
console.log(JSON.stringify(results, null, 2));