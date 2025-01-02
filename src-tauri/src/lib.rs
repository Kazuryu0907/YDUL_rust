// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod obs;
mod camera;

use tokio::sync::Mutex;
use tauri::{Manager, State};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn obs_login(state:State<'_,Mutex<obs::ObsClass>>,host: &str, port: u16, password: Option<&str>) -> Result<String,String> {
    let mut state = state.lock().await;
    state.login(host, port, password).await
}

#[tauri::command]
fn camera_preview(index:i32) -> Result<String,String>{
    let res = camera::show_camera(index);
    match res{
        Ok(_) => Ok("Preview".to_string()),
        Err(_) => Err("No camera".to_string())
    }
}
// #[tauri::command]
// async fn obs_start_virtual_cam(state:State<'_, Mutex<obs::ObsClass>>) -> Result<String,String> {
//     let state = state.lock().await;
//     state.set_virtual_cam().await
// }

#[tauri::command]
async fn obs_start(state:State<'_, Mutex<obs::ObsClass>>) -> Result<String,String>{
    let state = state.lock().await;
    state.set_virtual_cam().await?;
    state.set_replay_buffer().await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(obs::ObsClass::new()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet,obs_login,obs_start,camera_preview])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
