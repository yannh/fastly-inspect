// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit, {
  run
} from "./pkg/fastly_inspect.js";

var app = new Vue({
  el: '#app',
  data: {
    client_ip: '',
    client_asname: '',
    client_asnumber: '',
    resolver_ip: '',
    resolver_asname: '',
    resolver_asnumber: '',
    resolver_cc: ''
  }
})

const runWasm = async () => {
  const helloWorld = await wasmInit("./pkg/fastly_inspect_bg.wasm");
  const dns_infos = await run();
  console.log(dns_infos);
  app.client_ip = dns_infos.client_ip_info.ip;
  app.client_asname = dns_infos.client_ip_info.as_name;
  app.client_asnumber = dns_infos.client_ip_info.as_number;
  app.resolver_ip = dns_infos.dns_resolver_info.ip;
  app.resolver_asname = dns_infos.dns_resolver_info.as_name;
  app.resolver_asnumber = dns_infos.dns_resolver_info.as_number;
  app.resolver_cc = dns_infos.dns_resolver_info.cc;
};

runWasm();
