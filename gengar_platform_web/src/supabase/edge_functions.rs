use gengar_engine::{account_call::*, build_vars::*, error::*, json::*, networking::*};
use js_sys::Reflect;
use std::sync::{Arc, LazyLock, Mutex};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, Response};

const CHECKOUT_URL: &str = "/functions/v1/stripe_checkout_base";
const FETCH_USER_URL: &str = "/functions/v1/fetch_user_data";

pub async fn call_stripe_checkout_sandbox(user_auth_token: &str) -> NetworkCallStatus {
    let opts = RequestInit::new();
    opts.set_method("POST");

    // Set headers
    let headers = Headers::new().unwrap();
    headers
        .set("Authorization", &format!("Bearer {}", user_auth_token))
        .unwrap();
    headers.set("Content-Type", "application/json").unwrap();
    opts.set_headers(&headers);
    let url = format!("{}{}", server_env().supabase_url, CHECKOUT_URL);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    // Fetch call
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    let resp: Response = resp_value.dyn_into().unwrap();
    let status = resp.status();

    let text_promise = resp.text().unwrap();
    let text_jsvalue = JsFuture::from(text_promise).await.unwrap();
    let resp_str = text_jsvalue.as_string().unwrap();

    if status >= 400 {
        return NetworkCallStatus::Error {
            error: AccountError::UnknownError { response: resp_str },
        };
    }

    NetworkCallStatus::Success { response: resp_str }
}

pub async fn fetch_user_account(user_auth_token: &str) -> NetworkCallStatus {
    let opts = RequestInit::new();
    opts.set_method("POST");

    // Set headers
    let headers = Headers::new().unwrap();
    headers
        .set("Authorization", &format!("Bearer {}", user_auth_token))
        .unwrap();
    headers.set("Content-Type", "application/json").unwrap();
    opts.set_headers(&headers);

    let url = format!("{}{}", server_env().supabase_url, FETCH_USER_URL);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();

    // Fetch call
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    let resp: Response = resp_value.dyn_into().unwrap();
    let status = resp.status();

    let text_promise = resp.text().unwrap();
    let text_jsvalue = JsFuture::from(text_promise).await.unwrap();
    let resp_str = text_jsvalue.as_string().unwrap();

    if status >= 400 {
        return NetworkCallStatus::Error {
            error: AccountError::UnknownError { response: resp_str },
        };
    }

    NetworkCallStatus::Success { response: resp_str }
}
