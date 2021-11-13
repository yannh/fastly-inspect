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
        pops_latency: [],
    },
    filters: {
        capitalize: function (value) {
            if (!value) return ''
            value = value.toString()
            return value.charAt(0).toUpperCase() + value.slice(1)
        }
    },
    computed: {
        sortedPOPs: function() {
            function compare(a, b) {
                if (a.pop < b.pop)
                    return -1;
                if (a.pop > b.pop)
                    return 1;
                return 0;
            }

            return this.pops_latency.sort(compare);
        }
    }
})

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

const runWasm = async () => {
    await wasmInit("./lib/fastly_inspect_bg.wasm");

    debug_resolver_js().then(res => {
        app.client_ip_info = res.client_ip_info;
        app.dns_resolver_info = res.dns_resolver_info;
    });
    perf_map_config_js().then(res => {
        app.geo_ip = res.geo_ip;

        res.pops.map(pop => {
            const url = `https://${pop.popId}.pops.fastly-analytics.com/test_object.svg?unique=1636811062430p1v53fsd-perfmap&popId=${pop.popId}`;
            fetch(url).then(_ => {
                if (performance === undefined) {
                    log("= Calculate Load Times: performance NOT supported");
                    return;
                }
                setTimeout(function() { // There seems to be a small race condition until the timings are available
                    const resources = performance.getEntriesByType("resource");
                    const pop_timings = resources.find(r => r.name === url);
                    app.pops_latency.push({"pop": pop.popId, "latency": pop_timings.responseStart - pop_timings.requestStart})
                    console.log("timing for "+url+" : " + pop_timings + "\n\n object was:"+JSON.stringify(resources));
                }, 1000);
            })
        })
    });

    req_infos_js(location.protocol + "//" + location.hostname + ":" + location.port).then(res => {
        app.req_infos = res;
    });

    // tcpinfo_js().then(res => {
    //     app.tcpinfo = res;
    // });
};

runWasm();