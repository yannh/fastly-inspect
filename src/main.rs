use futures::executor::block_on;
use fastly_inspect::{debug_resolver, perf_map_config};
use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct Client {
    ip: String,
    as_name: String,
    country: String,
}

#[derive(Debug, Serialize)]
pub struct Output {
    pub client: Client
}

fn main() {
    let mut o =  Output{
        client: Client {
            as_name: String::from(""),
            country: String::from(""),
            ip: String::from(""),
        }
    };

    match block_on(debug_resolver()) {
        Ok(res) => {
            o.client.ip = res.client_ip_info.ip;
            o.client.as_name = res.client_ip_info.as_name;
        }
        Err(e) => eprintln!("{}", e),
    };

    match block_on(perf_map_config()) {
        Ok(res) => {
            o.client.country = res.geo_ip.ct;
        }
        Err(e) => eprintln!("{}", e),
    };

    match serde_json::to_string_pretty(&o) {
        Ok(r) => println!("{}", r),
        Err(e) => eprintln!("{}", e),
    }

}
