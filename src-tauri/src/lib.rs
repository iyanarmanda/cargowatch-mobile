// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde_json::Value;
use std::sync::Mutex;

static FCM_TOKEN: Mutex<Option<String>> = Mutex::new(None);

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_com_iyanarmanda_cargowatch_MainActivity_setFcmToken(
    mut env: jni::JNIEnv,
    _this: jni::objects::JObject,
    token: jni::objects::JString,
) {
    if let Ok(token_str) = env.get_string(&token) {
        let token_string: String = token_str.into();
        if let Ok(mut lock) = FCM_TOKEN.lock() {
            *lock = Some(token_string);
        }
    }
}

#[tauri::command]
async fn get_fcm_token() -> Result<String, String> {
    #[cfg(target_os = "android")]
    {
        for _ in 0..10 {
            if let Ok(lock) = FCM_TOKEN.lock() {
                if let Some(token) = lock.as_ref() {
                    return Ok(token.clone());
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        Err("FCM Token request timeout".to_string())
    }

    #[cfg(not(target_os = "android"))]
    {
        Ok("mock_desktop_token".to_string())
    }
}

#[tauri::command]
async fn get_request(url: String) -> Result<Value, String> {
    let response = reqwest::get(&url)
        .await
        .map_err(|e| format!("Failed to fetch API: {}", e))?
        .json::<Value>()
        .await
        .map_err(|e| format!("Failed to read JSON: {}", e))?;

    Ok(response)
}

#[tauri::command]
async fn post_request(url: String, payload: Value) -> Result<Value, String> {
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch API: {}", e))?;

    let status = response.status();
    let body: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to read JSON: {}", e))?;

    if !status.is_success() {
        let error_msg = body
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("Server error");
        return Err(error_msg.to_string());
    }

    Ok(body)
}

#[tauri::command]
async fn patch_request(url: String, payload: Value) -> Result<Value, String> {
    let client = reqwest::Client::new();
    let response = client
        .patch(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch API: {}", e))?;

    let status = response.status();
    let body: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to read JSON: {}", e))?;

    if !status.is_success() {
        let error_msg = body
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("Server error");
        return Err(error_msg.to_string());
    }
    Ok(body)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_request,
            post_request,
            patch_request,
            get_fcm_token,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
