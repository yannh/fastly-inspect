// Import our outputted wasm ES6 module
// Which, export default's, an initialization function
import wasmInit, {
    fastly_inspect_js
} from "./fastly_inspect.js";

var app = new Vue({
    el: '#app',
    data: {
        fastly_inspect: {
            geoip: {},
            popLatency: {},
            popAssignments: {},
            request: {},
        },
    },
    filters: {
        capitalize: function (value) {
            if (!value) return ''
            value = value.toString()
            return value.charAt(0).toUpperCase() + value.slice(1)
        },
        base64: function (s) {
            return btoa(JSON.stringify(s, null, 2));
        }
    },
    computed: {
        sortedPOPs: function() {
            return new Map([...this.fastly_inspect.popLatency.entries()].sort());
        }
    },
})

const runWasm = async () => {
    await wasmInit("./lib/fastly_inspect_bg.wasm");

    fastly_inspect_js(location.protocol + "//" + location.hostname + ":" + location.port).then(async res => {
        app.fastly_inspect = res;

        var pl = Object.entries(res.popLatency);
        console.log(pl);
        while (pl.length) {
            await Promise.all(pl.splice(0, 2).map(async pop => { // Only 2 requests at a time, to not skew the timings
                const url = `https://${pop[0]}.pops.fastly-analytics.com/test_object.svg?unique=1636811062430p1v53fsd-perfmap&popId=${pop[0]}`;
                await fetch(url).then(_ => {
                    if (performance === undefined) {
                        return;
                    }
                    setTimeout(function() { // There seems to be a small race condition until the timings are available
                        const resources = performance.getEntriesByType("resource");
                        const pop_timings = resources.find(r => r.name === url);
                        app.fastly_inspect.popLatency[pop[0]] = pop_timings.responseStart - pop_timings.requestStart;
                    }, 1000);
                })
            }));
        }
    });

    //perf_map_config_js().then(async res => {
    //    app.geo_ip = res.geo_ip;

    //});

    // tcpinfo_js().then(res => {
    //     app.tcpinfo = res;
    // });
};

runWasm();