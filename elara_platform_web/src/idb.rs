use rexie::TransactionMode;
use std::sync::{LazyLock, Mutex};
use wasm_bindgen::{prelude::*, JsValue};

pub static LOADED_DATA: LazyLock<Mutex<Vec<u8>>> = LazyLock::new(|| Mutex::new(vec![]));

async fn get_db() -> rexie::Rexie {
    // Create a new database
    rexie::Rexie::builder("packworld")
        // Set the version of the database to 1.0
        .version(1)
        // Add an object store named `employees`
        .add_object_store(
            rexie::ObjectStore::new("saves")
                // Set the key path to `id`
                .key_path("files")
                .auto_increment(true),
        )
        // Build the database
        .build()
        .await
        .expect("Error opening idb")
}

pub async fn db_save(data: Vec<u8>) {
    let rexie = get_db().await;

    // Convert Vec<u8> to Uint8Array
    let uint8_array = js_sys::Uint8Array::from(&data[..]);

    // Start a transaction
    let tx = rexie
        .transaction(&["saves"], TransactionMode::ReadWrite)
        .unwrap();
    let store = tx.store("saves").unwrap();

    // Create an entry with an ID
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &JsValue::from_str("files"), &JsValue::from_f64(1.0)).unwrap();
    js_sys::Reflect::set(&obj, &JsValue::from_str("data"), &uint8_array).unwrap();

    store.put(&obj, None).await.unwrap();

    tx.commit().await.unwrap();

    super::log("Wrote save file to idb");
}

pub async fn db_load() {
    let rexie = get_db().await;

    let tx = rexie
        .transaction(&["saves"], TransactionMode::ReadOnly)
        .unwrap();
    let store = tx.store("saves").unwrap();

    // unwrap().unwrap();
    // Retrieve the stored entry using the ID (1)
    let result_first = match store.get(JsValue::from_f64(1.0)).await {
        Ok(v) => v,

        Err(error) => {
            super::log("Error getting save file.");
            return;
        }
    };

    let result = match result_first {
        Some(v) => v,
        None => {
            super::log("Couldn't find save file.");
            return;
        }
    };

    // Check if the result is valid
    if let Some(obj) = result.dyn_into::<js_sys::Object>().ok() {
        // Extract the `data` (which is a Uint8Array) from the object
        if let Some(data) = js_sys::Reflect::get(&obj, &JsValue::from_str("data"))
            .ok()
            .and_then(|d| d.dyn_into::<js_sys::Uint8Array>().ok())
        {
            let rust_bytes = data.to_vec();
            super::log("Retrieved save file from idb");
            match LOADED_DATA.lock() {
                Ok(mut v) => {
                    *v = rust_bytes;
                }
                Err(e) => {
                    super::log("Loaded data locked. Probably Already loading.");
                }
            }
        } else {
            super::log(&format!("Failed to retrieve 'data' field."));
        }
    } else {
        super::log(&format!("Failed to retrieve object."));
    }
}
