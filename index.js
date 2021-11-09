// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit, {
  run
} from "./pkg/fastly_debug_rust.js";

const runWasm = async () => {
  // Instantiate our wasm module
  const helloWorld = await wasmInit("./pkg/fastly_debug_rust_bg.wasm");

  // Call the Add function export from wasm, save the result
  const addResult = await run();

  // Set the result onto the body
  document.body.textContent = JSON.stringify(addResult);
};
runWasm();
