use serde::{Deserialize, Serialize};
use surf::http::{Method, Url};
use rand::{thread_rng,Rng};
use rand::prelude::Distribution;
use std::collections::HashMap;
use futures::{future::FutureExt, pin_mut, select};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Instant};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use js_sys::Date;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugInfo {
    pub dns_resolver_info: DnsResolverInfo,
    pub client_ip_info: ClientIpInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnsResolverInfo {
    pub ip: String,
    pub as_name: String,
    pub as_number: String,
    pub cc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientIpInfo {
    pub ip: String,
    pub as_name: String,
    pub as_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerfMapConfig {
    pub geo_ip: GeoIP,
    pub pops: Vec<Pop>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pop {
    #[serde(rename = "popId")]
    pub pop_id: String,
    pub hostname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoIP {
    pub ci: String,
    pub co: String,
    pub ct: String,
    pub st: String,
    pub c_ip: String,
    pub c_asn: String,
    pub c_asn_name: String,
    pub r_ip: String,
    pub r_asn: String,
    pub r_asn_name: String,
    pub r_ci: String,
    pub r_co: String,
    pub r_ct: String,
    pub r_st: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqInfos {
    pub accept: String,
    pub accept_encoding: String,
    pub accept_language: String,
    pub host: String,
    pub pop: String,
    pub server: String,
    pub user_agent: String,
    pub x_forwarded_for: String,
    pub time: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReqInfosLegacy {
    pub cwnd: u32,
    pub nexthop: String,
    pub rtt: u32,
    pub delta_retrans: u32,
    pub total_retrans: u32,
    pub client_ip: String,
    pub client_as_name: String,
    pub client_as_number: String,
    pub city: String,
    pub continent: String,
    pub country: String,
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FastlyInspectRequest {
    pub resolver_ip: String,
    pub resolver_as_name: String,
    pub resolver_as_number: String,
    pub resolver_country_code: String,
    pub client_ip: String,
    pub client_as_name: String,
    pub client_as_number: String,
    pub time: String,
    pub host: String,
    pub accept: String,
    pub useragent: String,
    pub acceptlanguage: String,
    pub acceptencoding: String,
    pub fastlyserverip: String,
    pub xff: String,
    pub datacenter: String,
    pub bandwidth_mbps: f64,
    pub cwnd: u32,
    pub nexthop: String,
    pub rtt: u32,
    pub delta_retrans: u32,
    pub total_retrans: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FastlyInspectPopAs {
    pub popname: String,
    pub hostname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FastlyInspectPopAssignments {
    pub ac: String,
    #[serde(rename = "as")]
    pub popas: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FastlyInspect {
    pub geoip: GeoIP,
    #[serde(rename = "popLatency")]
    pub pop_latency: HashMap<String, u16>,
    pub request: FastlyInspectRequest,
    #[serde(rename = "popAssignments")]
    pub pop_assignments: FastlyInspectPopAssignments,
}


// This is a copy of Alphanumeric but with only [a-z0-9]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
pub struct LowerCaseAlphanumeric;
impl Distribution<u8> for LowerCaseAlphanumeric {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u32 = 26 + 10;
        const GEN_ASCII_STR_CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz\
                0123456789";
        loop {
            let var = rng.next_u32() >> (32 - 6);
            if var < RANGE {
                return GEN_ASCII_STR_CHARSET[var as usize];
            }
        }
    }
}

#[cfg(feature = "alloc")]
impl DistString for LowerCaseAlphanumeric {
    fn append_string<R: Rng + ?Sized>(&self, rng: &mut R, string: &mut String, len: usize) {
        unsafe {
            let v = string.as_mut_vec();
            v.extend(self.sample_iter(rng).take(len));
        }
    }
}

fn gen_perfmaphost() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&LowerCaseAlphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    let dt = chrono::offset::Utc::now();
    let ts = dt.timestamp();
    let ms = dt.timestamp_subsec_millis() % 1000;

    return format!("{}{}{}-perfmap", ts, ms, rand_string);
}

pub async fn debug_resolver() -> Result<DebugInfo, surf::Error> {
    let url = Url::parse("https://1636492611342-jn6tpar-9z.u.fastly-analytics.com/debug_resolver")?;
    let client = surf::Client::new();
    let request = surf::Request::builder(Method::Get, url.clone())
        .header("Accept", "application/json")
        .header("Content-type", "text/plain")
        .build();
    Ok(client.recv_json(request).await?)
}


// Get data from a VCL service for vars not yet available in compute
pub async fn req_infos_legacy(hostname: &str) -> Result<ReqInfosLegacy, surf::Error> {
    let url = Url::parse(&*format!("{}/req_infos", hostname))?;
    let client = surf::Client::new();
    let request = surf::Request::builder(Method::Get, url.clone())
        .header("Accept", "application/json")
        .header("Content-type", "text/plain")
        .build();
    Ok(client.recv_json(request).await?)
}

pub async fn req_infos(client: &surf::Client, hostname: &str) -> Result<ReqInfos, surf::Error> {
    let url = Url::parse(&*format!("{}/api/req_infos", hostname))?;
    let request = surf::Request::builder(Method::Get, url.clone())
        .header("Accept", "application/json")
        .header("Content-type", "text/plain");

    // Set user-agent for CLI
    #[cfg(not(target_arch = "wasm32"))]
    let request = request.header("User-Agent", "Fastly-Inspect v0.1.2");

    Ok(client.recv_json(request.build()).await?)
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn speed_test(client: &surf::Client, hostname: &str) -> Result<f64, surf::Error> {
    let url = Url::parse(&*format!("{}/api/speed_test", hostname))?;
    let request = surf::Request::builder(Method::Get, url.clone())
        .header("Accept", "application/json")
        .header("Content-type", "text/plain")
        .build();

    let now = Instant::now();
    let _ = client.recv_bytes(request).await;
    let elapsed = now.elapsed().as_millis();

    let bits_loaded_mb = 4; // 0.5MB
    return Ok(bits_loaded_mb as f64 * 1000 as f64 / elapsed as f64);
}

#[cfg(target_arch = "wasm32")]
pub async fn speed_test(client: &surf::Client, hostname: &str) -> Result<f64, surf::Error> {
    let url = Url::parse(&*format!("{}/api/speed_test", hostname))?;
    let request = surf::Request::builder(Method::Get, url.clone())
        .header("Accept", "application/json")
        .header("Content-type", "text/plain")
        .build();

    let b = Date::now();
    let _ = client.recv_bytes(request).await;
    let e = Date::now();
    let elapsed = e-b;

    let bits_loaded_mb = 4; // 0.5MB
    return Ok(bits_loaded_mb as f64 * 1000 as f64 / elapsed as f64);
}


pub async fn perf_map_config() -> Result<PerfMapConfig, surf::Error> {
    let dyn_host = gen_perfmaphost();
    let url = Url::parse(&*format!("https://{}.u.fastly-analytics.com/perfmapconfig.js?jsonp=removeme", dyn_host))?;
    let client = surf::Client::new();
    let request = surf::Request::builder(Method::Get, url.clone())
        .header("Accept", "application/json")
        .header("Content-type", "text/plain")
        .build();

    let b = client.recv_bytes(request).await?;
    let mut res: String = b.iter().map(|&c| c as char).collect();

    let offset = res.find('\n').unwrap_or(res.len());
    res.replace_range(..offset, "");
    res.pop();
    res.pop();
    res = res.replace("'", "\"");
    match serde_json::from_str(&*res) {
        Ok(r) => Ok(r),
        Err(e) => Err(surf::Error::from_str(surf::StatusCode::Accepted, format!("{} : {}", e, res))),
    }
}

pub async fn pop_as(w: &str) -> Result<FastlyInspectPopAs, surf::Error> {
    let url = Url::parse(&*format!("https://{}.fastly-analytics.com/popname.js?jsonp=removeme&unique=16365577309317k96lvao-perfmap", w))?;
    let client = surf::Client::new();
    let request = surf::Request::builder(Method::Get, url.clone())
        .header("Accept", "application/json")
        .header("Content-type", "text/plain")
        .build();

    let b = client.recv_bytes(request).await?;
    let mut res: String = b.iter().map(|&c| c as char).collect();

    let offset = res.find('\n').unwrap_or(res.len());
    res.replace_range(..offset, "");
    res.pop();
    res.pop();
    res = res.replace("'", "\"");
    match serde_json::from_str(&*res) {
        Ok(r) => Ok(r),
        Err(e) => Err(surf::Error::from_str(surf::StatusCode::Accepted, format!("{} : {}", e, res))),
    }
}

pub async fn fastly_inspect(hostname: String, hostname_helper: String) -> Result<FastlyInspect, surf::Error> {
    let hostname = hostname.trim_end_matches("/");
    let hostname_helper = hostname_helper.trim_end_matches("/");

    let popl: HashMap<String, u16> = HashMap::new();
    let mut o = FastlyInspect{
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
            bandwidth_mbps: 0.0,
            cwnd: 0,
            nexthop: String::from(""),
            rtt: 0,
            delta_retrans: 0,
            total_retrans: 0
        }
    };

    let client = surf::Client::new();
    let perf_map_config_future =  perf_map_config().fuse();
    let req_infos_future = req_infos(&client, hostname).fuse();
    let req_infos_legacy_future = req_infos_legacy(hostname_helper).fuse();
    let debug_resolver_future = debug_resolver().fuse();
    let pop_as_ac_future = pop_as("ac").fuse();
    let pop_as_as_future = pop_as("ac").fuse();

    pin_mut!(perf_map_config_future, req_infos_future, req_infos_legacy_future, debug_resolver_future, pop_as_ac_future, pop_as_as_future);

    loop {
        select! {
            f = perf_map_config_future => {
                match f {
                    Ok (res) => {
                        o.geoip = res.geo_ip;
                        for pop in res.pops.iter() {
                            o.pop_latency.insert(pop.pop_id.clone(), 0);
                        }
                    }
                    Err (_) => println!("err"),
                }
            },

            f = req_infos_future => {
                match f {
                    Ok (res) => {
                        o.pop_assignments.popas = res.pop;
                        o.request.time = res.time;
                        o.request.accept = res.accept;
                        o.request.acceptlanguage = res.accept_language;
                        o.request.acceptencoding = res.accept_encoding;
                        o.request.host = res.host;
                        o.request.useragent = res.user_agent;
                        o.request.xff = res.x_forwarded_for;
                    }
                    Err (_) => println!("err"),
                }
            },

            f = req_infos_legacy_future => {
                match f {
                    Ok (res) => {
                        o.request.cwnd = res.cwnd;
                        o.request.delta_retrans = res.delta_retrans;
                        o.request.nexthop = res.nexthop;
                        o.request.total_retrans = res.total_retrans;
                        o.request.rtt = res.rtt;
                        o.request.client_ip = res.client_ip;
                        o.request.client_as_name = res.client_as_name;
                        o.request.client_as_number = res.client_as_number;
                    }
                    Err (_) => println!("err"),
                }
            },

            f = debug_resolver_future => {
                match f {
                    Ok (res) => {
                        o.request.resolver_ip = res.dns_resolver_info.ip;
                        o.request.resolver_as_name = res.dns_resolver_info.as_name;
                        o.request.resolver_as_number = res.dns_resolver_info.as_number;
                        o.request.resolver_country_code = res.dns_resolver_info.cc;
                    }
                    Err (_) => println!("err"),
                }
            },

            f = pop_as_ac_future => {
                match f {
                    Ok (res) => {
                        o.pop_assignments.ac = res.popname;
                    }
                    Err (_) => println!("err"),
                }
            },

            f = pop_as_as_future => {
                match f {
                    Ok (res) => {
                        o.pop_assignments.popas = res.popname.clone();
                        o.request.datacenter = res.popname;
                    }
                    Err (_) => println!("err"),
                }
            },

            complete => break,
        };
    }

    // Run this one separately, so it is not affected by other requests
    match speed_test(&client, hostname).await {
        Ok(res) => {
            o.request.bandwidth_mbps = res;
        },
        Err(e) => return Err(e),
    };

    return Ok(o);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
pub async fn fastly_inspect_js(hostname: String, hostname_helper: String) -> Result<JsValue, JsValue> {
    match fastly_inspect(hostname, hostname_helper).await {
        Ok(res) => return Ok(JsValue::from_serde(&res).unwrap()),
        Err(e) => return Err(JsValue::from(&*format!("error retrieving fastly_inspect: {}", e))),
    };
}