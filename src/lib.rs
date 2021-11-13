use serde::{Deserialize, Serialize};
use surf::http::{Method, Url};
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{thread_rng,Rng};
use rand::distributions::Alphanumeric;
use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TcpInfo {
    pub requests: u32,
    pub cwnd: u32,
    pub nexthop: String,
    pub rtt: u32,
    pub delta_retrans: u32,
    pub total_retrans: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FastlyInspect {
    pub geoip: GeoIP,
    #[serde(rename = "popLatency")]
    pub pop_latency: HashMap<String, String>,
}

fn gen_perfmaphost() -> String {
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    return format!("{:?}{}-perfmap", since_epoch, rand_string);
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

pub async fn req_infos(hostname: String) -> Result<ReqInfos, surf::Error> {
    let url = Url::parse(&*format!("{}/req_infos", hostname))?;
    let client = surf::Client::new();
    let request = surf::Request::builder(Method::Get, url.clone())
        .header("Accept", "application/json")
        .header("Content-type", "text/plain")
        .build();
    Ok(client.recv_json(request).await?)
}

pub async fn tcpinfo() -> Result<TcpInfo, surf::Error> {
    let url = Url::parse("https://fastly-helper.mandragor.org/tcpinfo")?;
    let client = surf::Client::new();
    let request = surf::Request::builder(Method::Get, url.clone())
        .header("Accept", "application/json")
        .header("Content-type", "text/plain")
        .build();
    Ok(client.recv_json(request).await?)
}

pub async fn perf_map_config() -> Result<PerfMapConfig, surf::Error> {
    let url = Url::parse("https://16365577309317k96lvao-perfmap.u.fastly-analytics.com/perfmapconfig.js?jsonp=removeme")?;
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

pub async fn fastly_inspect() -> Result<FastlyInspect, surf::Error> {
    let popl: HashMap<String, String> = HashMap::new();
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
    };

    match perf_map_config().await {
        Ok(res) => {
            o.geoip = res.geo_ip;
            for pop in res.pops.iter() {
            }

        },
        Err(e) => return Err(e),
    };

    return Ok(o);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
pub async fn debug_resolver_js() -> Result<JsValue, JsValue> {
    match debug_resolver().await {
        Ok(res) => return Ok(JsValue::from_serde(&res).unwrap()),
        Err(_) => return Err(JsValue::from("error retrieving debug_resolver")),
    };
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
pub async fn perf_map_config_js() -> Result<JsValue, JsValue> {
    match perf_map_config().await {
        Ok(res) => return Ok(JsValue::from_serde(&res).unwrap()),
        Err(e) => return Err(JsValue::from(format!("error retrieving perf_map_config: {}", e))),
    };
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
pub async fn req_infos_js(hostname: String) -> Result<JsValue, JsValue> {
    match req_infos(hostname).await {
        Ok(res) => return Ok(JsValue::from_serde(&res).unwrap()),
        Err(e) => return Err(JsValue::from(format!("error retrieving req_infos: {}", e))),
    };
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
#[cfg(target_arch = "wasm32")]
pub async fn tcpinfo_js() -> Result<JsValue, JsValue> {
    match tcpinfo().await {
        Ok(res) => return Ok(JsValue::from_serde(&res).unwrap()),
        Err(e) => return Err(JsValue::from(format!("error retrieving tcpinfo: {}", e))),
    };
}
