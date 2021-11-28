use std::collections::HashMap;
use futures::executor::block_on;
use fastly_inspect::{fastly_inspect, FastlyInspect, GeoIP, FastlyInspectRequest, FastlyInspectPopAssignments};
use ttfb::ttfb;

fn main() {
    let popl: HashMap<String, u16> = HashMap::new();
    let mut fi = FastlyInspect{
        geoip: GeoIP {
            ci: String::from(""),
            co: String::from(""),
            ct: String::from(""),
            st: String::from(""),
            c_ip: String::from(""),
            c_asn: String::from(""),
            c_asn_name: String::from(""),
            r_ip: String::from(""),
            r_asn: String::from(""),
            r_asn_name: String::from(""),
            r_ci: String::from(""),
            r_co: String::from(""),
            r_ct: String::from(""),
            r_st: String::from(""),
        },
        pop_latency: popl,
        pop_assignments: FastlyInspectPopAssignments {
            ac: String::from(""),
            popas: String::from(""),
        },
        request: FastlyInspectRequest{
            resolver_ip: String::from(""),
            resolver_as_name: String::from(""),
            resolver_as_number: String::from(""),
            resolver_country_code: String::from(""),
            client_ip: String::from(""),
            client_as_name: String::from(""),
            client_as_number: String::from(""),
            time: String::from(""),
            host: String::from(""),
            accept: String::from(""),
            useragent: String::from(""),
            acceptlanguage: String::from(""),
            acceptencoding: String::from(""),
            fastlyserverip: String::from(""),
            xff: String::from(""),
            datacenter: String::from(""),
            bandwidth_mbps: String::from(""),
            cwnd: 0,
            nexthop: String::from(""),
            rtt: 0,
            delta_retrans: 0,
            total_retrans: 0
        }
    };

    match block_on(fastly_inspect(String::from("https://fastly-inspect.edgecompute.app/"))) {
        Ok(res) => {
            fi = res;
        }
        Err(e) => eprintln!("{}", e),
    };

    for (pop, popl) in fi.pop_latency.iter_mut() {
        let url = format!("https://{}.pops.fastly-analytics.com/test_object.svg?unique=1636811062430p1v53fsd-perfmap&popId={}", pop, pop);
        match ttfb(url, false) {
            Ok(l) =>  *popl = l.http_ttfb_duration_rel().as_millis() as u16,
            Err (e) =>  eprintln!("{}", e),
        }
    }

    match serde_json::to_string_pretty(&fi) {
        Ok(r) => println!("{}", r),
        Err(e) => eprintln!("{}", e),
    }
}
