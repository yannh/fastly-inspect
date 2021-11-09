// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit, {
  run
} from "./pkg/fastly_inspect.js";

const runWasm = async () => {
  const helloWorld = await wasmInit("./pkg/fastly_inspect_bg.wasm");
  const dns_infos = await run();
  console.log(dns_infos);
  document.getElementById("client_ip").textContent = dns_infos.client_ip_info.ip;
  document.getElementById("client_asname").textContent = dns_infos.client_ip_info.as_name;
  document.getElementById("resolver_ip").textContent = dns_infos.dns_resolver_info.ip;
  document.getElementById("resolver_asname").textContent = dns_infos.dns_resolver_info.as_name;
};

runWasm();
