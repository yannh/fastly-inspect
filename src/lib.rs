use serde::{Deserialize, Serialize};
use surf::http::{Method, Url};

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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeoIP {
    pub ci: String,
    pub co: String,
    pub ct: String,
    pub st: String,
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
