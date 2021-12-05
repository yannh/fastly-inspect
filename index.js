import wasmInit, {
    fastly_inspect_js
} from "./fastly_inspect.js";

var app = new Vue({
    el: '#app',
    data: {
        fastly_inspect: {
            geoip: {},
            popLatency: {},
            popAssignments: {
                ac: "",
                as: "",
            },
            request: {
                bandwidth_mbps: 0.0,
            },
        },
    },
    filters: {
        capitalize: function (value) {
            if (!value) return '';
            value = value.toString();
            return value.charAt(0).toUpperCase() + value.slice(1);
        },
        base64: function (s) {
            return btoa(JSON.stringify(s, null, 2));
        },
        sortHash: function (h) {
            var resultHash = {};
            Object.keys(h).sort().forEach(k => resultHash[k]=h[k]);
            return resultHash;
        },
        formatDateTime: function (t) {
            if (!t) return '';
            return new Date(t).toLocaleString();
        }
    },
    methods: {
        toclipboard: function (e) {
            navigator.clipboard.writeText(this.$options.filters.base64(this.fastly_inspect));
        }
    }
})

const runWasm = async () => {
    await wasmInit("./lib/fastly_inspect_bg.wasm");

    fastly_inspect_js(location.protocol + "//" + location.hostname + ":" + location.port, "https://fastly-helper.mandragor.org/").then(async res => {
        app.fastly_inspect = res;

        var pl = Object.entries(res.popLatency);
        while (pl.length) {
            await Promise.all(pl.splice(0, 2).map(async pop => { // Only 2 requests at a time, to not skew the timings
                const url = `https://${pop[0]}.pops.fastly-analytics.com/test_object.svg?unique=1636811062430p1v53fsd-perfmap&popId=${pop[0]}`;
                await fetch(url).then(res => {
                    if (performance === undefined) {
                        return;
                    }

                    // Avoid garbage collection under chrome
                    // https://github.com/whatwg/fetch/issues/810
                    res.clone();

                    setTimeout(function() { // There seems to be a small race condition until the timings are available
                        const resources = performance.getEntriesByType("resource");
                        const pop_timings = resources.find(r => r.name === url);
                        // TTFB, minus TCP & SSL negociation
                        app.fastly_inspect.popLatency[pop[0]] = Math.round(pop_timings.responseStart - pop_timings.requestStart);
                    }, 1000);
                })
            }));
        }
    });
};

runWasm();