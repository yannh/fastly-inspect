use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{RequestInit, RequestMode};
use surf::http::{Method, Url};
/// A struct to hold some data from the github Branch API.
///
/// Note how we don't have to define every member -- serde will ignore extra
/// data when deserializing
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

// Let's define our external function (imported from JS)
// Here, we will define our external `console.log`
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub async fn debug_resolver() -> Result<DebugInfo, surf::Error> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = Url::parse("https://1636492611342-jn6tpar-9z.u.fastly-analytics.com/debug_resolver")?;
    let client = surf::Client::new();
    let request = surf::Request::builder(Method::Get, url.clone())
        .header("Accept", "application/json")
        .header("Content-type", "text/plain")
        .build();
    let res :DebugInfo = client.recv_json(request).await?;

    Ok(res)
}

#[wasm_bindgen]
pub async fn run() -> Result<JsValue, JsValue> {
    match debug_resolver().await {
        Ok(res) => return Ok(JsValue::from_serde(&res).unwrap()),
        Err(_) => return Err(JsValue::from("test2")),
    };
}
