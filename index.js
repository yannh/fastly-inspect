// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit, {
    debug_resolver_js, perf_map_config_js, req_infos_js, tcpinfo_js
} from "./fastly_inspect.js";

var app = new Vue({
    el: '#app',
    data: {
        client_ip_info: {},
        dns_resolver_info : {},
        geo_ip: {},
        req_infos: {},
        tcpinfo: {},
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
    await wasmInit("./lib/fastly_inspect_bg.wasm");

    debug_resolver_js().then(res => {
        app.client_ip_info = res.client_ip_info;
        app.dns_resolver_info = res.dns_resolver_info;
    });

    perf_map_config_js().then(res => {
        app.geo_ip = res.geo_ip;
    });

    req_infos_js(location.protocol + "//" + location.hostname + ":" + location.port).then(res => {
        app.req_infos = res;
    });

    tcpinfo_js().then(res => {
        app.tcpinfo = res;
    });
};

runWasm();