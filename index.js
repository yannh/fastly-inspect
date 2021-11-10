// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit, {
  debug_resolver_js, perf_map_config_js
} from "./pkg/fastly_inspect.js";

var app = new Vue({
  el: '#app',
  data: {
    client_ip: '',
    client_city : '',
    client_country : '',
    client_asname: '',
    client_asnumber: '',
    client_state: '',
    client_continent: '',
    resolver_ip: '',
    resolver_asname: '',
    resolver_asnumber: '',
    resolver_cc: '',
  },
  filters: {
    capitalize: function (value) {
      if (!value) return ''
      value = value.toString()
      return value.charAt(0).toUpperCase() + value.slice(1)
    }
  }
})

const runWasm = async () => {
  await wasmInit("./pkg/fastly_inspect_bg.wasm");

  const dns_infos = await debug_resolver_js();
  app.client_ip = dns_infos.client_ip_info.ip;
  app.client_asname = dns_infos.client_ip_info.as_name;
  app.client_asnumber = dns_infos.client_ip_info.as_number;
  app.resolver_ip = dns_infos.dns_resolver_info.ip;
  app.resolver_asname = dns_infos.dns_resolver_info.as_name;
  app.resolver_asnumber = dns_infos.dns_resolver_info.as_number;
  app.resolver_cc = dns_infos.dns_resolver_info.cc;

  const pmc = await perf_map_config_js();
  console.log(pmc)
  app.client_city = pmc.geo_ip.ci;
  app.client_country = pmc.geo_ip.ct;
  app.client_state = pmc.geo_ip.st;
  app.client_continent = pmc.geo_ip.co;
};

runWasm();
