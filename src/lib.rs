use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen::JsCast;

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
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ClientIpInfo {
    pub ip: String,
    pub as_name: String,
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

#[wasm_bindgen]
pub async fn run() -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let url = "https://1636492611342-jn6tpar-9z.u.fastly-analytics.com/debug_resolver";
    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Use serde to parse the JSON into a struct.
    let branch_info: DebugInfo = json.into_serde().unwrap();

    // Send the `Branch` struct back to JS as an `Object`.
    Ok(JsValue::from_serde(&branch_info).unwrap())
}
