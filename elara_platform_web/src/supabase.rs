use js_sys::Reflect;
use elara_engine::{account_call::*, build_vars::*, error::*, json::*, networking::*};
use std::sync::{Arc, LazyLock, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, Response};

pub mod edge_functions;

pub use edge_functions::*;

const ONE_TIME_PASSWORD_URL: &str = "/auth/v1/otp";
const VERIFY_URL: &str = "/auth/v1/verify";
const TOKEN_REFRESH_URL: &str = "/auth/v1/token?grant_type=refresh_token";

pub static ACCOUNT_ERROR: LazyLock<Mutex<Option<AccountError>>> =
    LazyLock::new(|| Mutex::new(None));

fn supa_to_account_error(input: String) -> Result<AccountError, Error> {
    let json = elara_engine::json::load(&input)?;
    let error_code = json
        .get(vec!["error_code".to_string()])
        .ok_or(Error::JsonMissingEntry)?
        .as_string()?;

    if error_code == "validation_failed" {
        Ok(AccountError::EmailInvalid)
    } else if error_code == "email_address_invalid" {
        Ok(AccountError::EmailInvalid)
    } else {
        Ok(AccountError::UnknownError { response: input })
    }
}

/*
// supabase storage api info
// https://stackoverflow.com/questions/75540112/how-to-upload-to-supabase-storage-using-curl
async fn upload_data(data: Vec<u8>, user_id: String) {
    /*
    let opts = RequestInit::new();
    opts.set_method("POST");

    opts.set_body(&JsValue::from_str(unsafe {
        std::str::from_utf8_unchecked(&data)
    }));

    let headers = Headers::new().unwrap();
    headers.set("apikey", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InFxaWJxamxndmtoenlyamFhYnZnIiwicm9sZSI6ImFub24iLCJpYXQiOjE3NDIzMTc1MTUsImV4cCI6MjA1Nzg5MzUxNX0.wYCDHY5jXVIex2E6ZmzU16DQC5GtqMiPV974N7TQKUM").unwrap();
    headers
    .set("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InFxaWJxamxndmtoenlyamFhYnZnIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc0MjMxNzUxNSwiZXhwIjoyMDU3ODkzNTE1fQ.uNXhoOMoAKyjcN2A2Iss1AIwCns46V9abIaGC_luQBk")
    .unwrap();
    headers.set("x-upsert", "true").unwrap();

    opts.set_headers(&headers);
    */

    // Create a new JavaScript object for RequestInit
    let request_init = js_sys::Object::new();
    Reflect::set(
        &request_init,
        &JsValue::from_str("method"),
        &JsValue::from_str("POST"),
    )
    .unwrap();
    Reflect::set(
        &request_init,
        &JsValue::from_str("keepalive"),
        &JsValue::from(js_sys::Boolean::from(true)),
    )
    .unwrap();

    // Create and set headers
    let headers = Headers::new().unwrap();
    headers.set("apikey", API_KEY).unwrap();
    headers
    .set("Authorization", "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InFxaWJxamxndmtoenlyamFhYnZnIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc0MjMxNzUxNSwiZXhwIjoyMDU3ODkzNTE1fQ.uNXhoOMoAKyjcN2A2Iss1AIwCns46V9abIaGC_luQBk")
    .unwrap();
    headers.set("x-upsert", "true").unwrap();

    Reflect::set(
        &request_init,
        &JsValue::from_str("headers"),
        &headers.into(),
    )
    .unwrap();

    let url = format!(
        "https://qqibqjlgvkhzyrjaabvg.supabase.co/storage/v1/object/saves-public/{}.gsf",
        user_id,
    );

    let request = Request::new_with_str_and_init(&url, &request_init.unchecked_into()).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    super::log("Save upload successful");
}

async fn download_data(user_id: String) {
    let opts = RequestInit::new();
    opts.set_method("GET");

    // generate random string to force invalidate the cache
    let cache_buster: String = web_sys::window().unwrap().crypto().unwrap().random_uuid();

    let url = format!(
        "https://qqibqjlgvkhzyrjaabvg.supabase.co/storage/v1/object/saves-public//{}.gsf?bust={}",
        user_id, cache_buster
    );

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let buf_val = JsFuture::from(resp.array_buffer().unwrap()).await.unwrap();

    let typebuf: js_sys::Uint8Array = js_sys::Uint8Array::new(&buf_val);

    let mut body = vec![0; typebuf.length() as usize];
    typebuf.copy_to(&mut body[..]);

    todo!("put the save in the global vec and then in engine_state.game_to_laod");

    super::log("download successful ");
}
*/

pub async fn send_otp(email: String) -> NetworkCallStatus {
    let json_str = format!("{{\"email\":\"{}\"}}", email);

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_body(&JsValue::from_str(&json_str));

    let headers = Headers::new().unwrap();
    headers
        .set("apikey", server_env().supabase_api_key)
        .unwrap();

    opts.set_headers(&headers);

    // generate random string to force invalidate the cache
    // let cache_buster: String = web_sys::window().unwrap().crypto().unwrap().random_uuid();

    let url = format!("{}{}", server_env().supabase_url, ONE_TIME_PASSWORD_URL);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let text_promise = resp.text().unwrap();
    let text_jsvalue = JsFuture::from(text_promise).await.unwrap();
    let resp_str = text_jsvalue.as_string().unwrap();

    let status: u16 = resp.status();

    if status == 400 {
        return NetworkCallStatus::Error {
            error: supa_to_account_error(resp_str).unwrap(),
        };
    } else {
        return NetworkCallStatus::Success { response: resp_str };
    }
}

pub async fn verify_pairing_code(pairing_code: String, email: String) -> NetworkCallStatus {
    let json_str = format!(
        "{{\"token\":\"{}\",\"type\":\"email\",\"email\":\"{}\"}}",
        pairing_code, email
    );

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_body(&JsValue::from_str(&json_str));

    let headers = Headers::new().unwrap();
    headers
        .set("apikey", server_env().supabase_api_key)
        .unwrap();

    opts.set_headers(&headers);

    // generate random string to force invalidate the cache
    // let cache_buster: String = web_sys::window().unwrap().crypto().unwrap().random_uuid();

    let url = format!("{}{}", server_env().supabase_url, VERIFY_URL);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let text_promise = resp.text().unwrap();
    let text_jsvalue = JsFuture::from(text_promise).await.unwrap();
    let resp_str = text_jsvalue.as_string().unwrap();

    let status: u16 = resp.status();

    if status == 400 {
        return NetworkCallStatus::Error {
            error: supa_to_account_error(resp_str).unwrap(),
        };
    } else {
        return NetworkCallStatus::Success { response: resp_str };
    }
}

pub async fn exchange_refresh_token(refresh_token: String) -> NetworkCallStatus {
    super::log("exchanging refresh token");

    let json_str = format!("{{\"refresh_token\":\"{}\"}}", refresh_token);

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_body(&JsValue::from_str(&json_str));

    let headers = Headers::new().unwrap();
    headers
        .set("apikey", server_env().supabase_api_key)
        .unwrap();

    opts.set_headers(&headers);

    // generate random string to force invalidate the cache
    // let cache_buster: String = web_sys::window().unwrap().crypto().unwrap().random_uuid();

    let url = format!("{}{}", server_env().supabase_url, TOKEN_REFRESH_URL);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    let text_promise = resp.text().unwrap();
    let text_jsvalue = JsFuture::from(text_promise).await.unwrap();
    let resp_str = text_jsvalue.as_string().unwrap();

    let status: u16 = resp.status();

    if status == 400 {
        return NetworkCallStatus::Error {
            error: supa_to_account_error(resp_str).unwrap(),
        };
    } else {
        return NetworkCallStatus::Success { response: resp_str };
    }
}
