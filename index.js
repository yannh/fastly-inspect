// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit, {
  run
} from "./pkg/fastly_inspect.js";

var app = new Vue({
  el: '#app',
  data: {
    message: 'Hello Vue!',
    clientip: '',
    client_asname: ''
  }
})

const runWasm = async () => {
  const helloWorld = await wasmInit("./pkg/fastly_inspect_bg.wasm");
  const dns_infos = await run();
  console.log(dns_infos);
  app.clientip = dns_infos.client_ip_info.ip;
  app.client_asname = dns_infos.client_ip_info.as_name;
  app.resolver_ip = dns_infos.dns_resolver_info.ip;
  app.resolver_asname = dns_infos.dns_resolver_info.as_name;
};

runWasm();
