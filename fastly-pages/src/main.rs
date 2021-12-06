use std::path::Path;

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};
use rust_embed::RustEmbed;
use std::ffi::OsStr;
use serde_json::json;
use chrono::prelude::*;
use base64::encode;

#[derive(RustEmbed)]
#[folder = "static/"]
struct Asset;

fn file_mimetype(filename: &str, default: mime::Mime) -> mime::Mime {
    let extension = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .map(|s| s.to_lowercase());

    let mime_wasm: mime::Mime = "application/wasm".parse().unwrap();
    match extension {
        Some(ext) => match ext.as_str() {
            "css" => mime::TEXT_CSS_UTF_8,
            "gif" => mime::IMAGE_GIF,
            "html" | "htm" => mime::TEXT_HTML_UTF_8,
            "jpeg" | "jpg" => mime::IMAGE_JPEG,
            "js" => mime::TEXT_JAVASCRIPT,
            "json" => mime::APPLICATION_JSON,
            "png" => mime::IMAGE_PNG,
            "svg" => mime::IMAGE_SVG,
            "wasm" => mime_wasm,
            _ => default,
        },
        _ => default,
    }
}

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Filter request methods...
    match req.get_method() {
        // Allow GET and HEAD requests.
        &Method::GET | &Method::HEAD => (),

        // Deny anything else.
        _ => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD")
                .with_body_text_plain("This method is not allowed\n"))
        }
    };

    const DEFAULT_MIMETYPE: mime::Mime = mime::APPLICATION_OCTET_STREAM;
    let mut filename = req.get_path().trim_start_matches("/");

    if filename == "api/req_infos" {
        let o = json! ({
            "accept": req.get_header_str(header::ACCEPT).unwrap_or(""),
            "accept_language": req.get_header_str(header::ACCEPT_LANGUAGE).unwrap_or(""),
            "accept_encoding": req.get_header_str(header::ACCEPT_ENCODING).unwrap_or(""),
            "host": req.get_header_str(header::HOST).unwrap_or(""),
            "user_agent": req.get_header_str(header::USER_AGENT).unwrap_or(""),
            "x_forwarded_for": req.get_header_str(header::FORWARDED).unwrap_or(""),
            "pop": std::env::var("FASTLY_POP").unwrap_or(String::new()),
            "server": std::env::var("FASTLY_HOSTNAME").unwrap_or(String::new()),
            "time": Utc::now(),
        });
        return Ok(Response::from_status(StatusCode::OK)
            .with_body_bytes(o.to_string().as_bytes())
            .with_content_type(file_mimetype(filename, mime::APPLICATION_JSON)));
    }

    if filename == "api/speed_test" {
        let o = "b".repeat(500*1024);
        return Ok(Response::from_status(StatusCode::OK)
            .with_body_bytes(o.to_string().as_bytes())
            .with_content_type(file_mimetype(filename, mime::APPLICATION_JSON)));
    }

    if filename == ""  || filename == "index.html" {
        filename = "index.html";
        match Asset::get(filename) {
            Some(asset) => {
                let html = std::str::from_utf8(asset.data.as_ref()).unwrap();
                let accept = req.get_header_str("Accept").unwrap_or("");
                let acceptlanguage = req.get_header_str("Accept-Language").unwrap_or("");
                let acceptencoding = req.get_header_str("Accept-Encoding").unwrap_or("");

                let html = html.replace("%BROWSERINFO%", &*encode(json!({
                    "accept": accept,
                    "acceptlanguage": acceptlanguage,
                    "acceptencoding": acceptencoding}).to_string()));

                return Ok(Response::from_status(StatusCode::OK)
                    .with_body_bytes(&html.as_bytes())
                    .with_content_type(file_mimetype(filename, DEFAULT_MIMETYPE)))
            },

            None => return Ok(Response::from_status(StatusCode::NOT_FOUND)
                .with_body_text_plain(&*format!("404 error, {} not found!", req.get_path()))),
        }
    }

    match Asset::get(filename) {
        Some(asset) => return Ok(Response::from_status(StatusCode::OK)
            .with_body_bytes(asset.data.as_ref())
            .with_content_type(file_mimetype(filename, DEFAULT_MIMETYPE))),

        None => return Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain(&*format!("404 error, {} not found!", req.get_path()))),
    }
}
