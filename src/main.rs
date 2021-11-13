use std::collections::HashMap;
use futures::executor::block_on;
use fastly_inspect::{fastly_inspect, FastlyInspect, GeoIP, Pop};
use serde::{Serialize};



fn main() {
    let popl: HashMap<String, String> = HashMap::new();
    let mut fi = FastlyInspect{
        geoip: GeoIP{
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
    };

    match block_on(fastly_inspect()) {
        Ok(res) => {
            fi = res;
        }
        Err(e) => eprintln!("{}", e),
    };

    match serde_json::to_string_pretty(&fi) {
        Ok(r) => println!("{}", r),
        Err(e) => eprintln!("{}", e),
    }
}
