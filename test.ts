import { get_mtsp } from "./lib/mtsp.wasm";


// const items = JSON.stringify([
//   { name: "A", addr: "addr1", lng: 127.0, lat: 37.5, g: 0 },
//   { name: "B", addr: "addr2", lng: 127.1, lat: 37.6, g: 0 },
// ]);


// console.log(get_mtsp(items, 2));



// const wasmBytes = await Deno.readFile("./lib/mtsp.wasm");
// const wasmModule = await WebAssembly.instantiate(wasmBytes, {});
// const { get_mtsp } = wasmModule.instance.exports;


// Rust 함수 호출
const items = JSON.stringify([
  { name: "A", addr: "addr1", lng: 127.0, lat: 37.5, g: 0 },
  { name: "B", addr: "addr2", lng: 127.1, lat: 37.6, g: 0 },
]);
const resultPtr = get_mtsp(items, 2);
console.log(resultPtr);
