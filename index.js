// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit, {
  run
} from "./pkg/fastly_inspect.js";

const runWasm = async () => {
  const helloWorld = await wasmInit("./pkg/fastly_inspect_bg.wasm");
  document.body.textContent = JSON.stringify(await run());
};

runWasm();
