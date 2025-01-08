// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod obs;
// mod camera;
mod socket;

use std::os::windows::process::CommandExt;
use std::{
    env, fs,
    path::Path,
    process::{Command, Stdio},
    thread,
};

// use std::sync::{Arc, Mutex};
use tauri::{App, AppHandle, Emitter, Manager, State};
use tauri_plugin_log::{Target, TargetKind};
use tokio::sync::Mutex;

use futures_util::{pin_mut, Stream, StreamExt};
use obws::{events::Event, Client};

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn obs_login(
    state: State<'_, Mutex<obs::ObsClass>>,
    host: &str,
    port: u16,
    password: Option<&str>,
) -> Result<String, String> {
    let mut state = state.lock().await;
    state.login(host, port, password).await
}

#[tauri::command]
async fn obs_start(state: State<'_, Mutex<obs::ObsClass>>) -> Result<String, String> {
    let state = state.lock().await;
    state.set_virtual_cam().await?;
    state.set_replay_buffer().await
}

#[tauri::command]
fn delete_file(path: String) -> Result<String, String> {
    let res = fs::remove_file(&path);
    match res {
        Ok(_) => Ok(format!("success to remove {}", path).to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
async fn obs_event_listen(
    app: AppHandle,
    state: State<'_, Mutex<obs::ObsClass>>,
) -> Result<(), String> {
    let state = state.lock().await;

    let client = state.get_client().await?.clone();
    tokio::spawn(async move {
        let events = match client.events() {
            Ok(stream) => stream,
            Err(e) => return Err(e.to_string()),
        };
        pin_mut!(events);
        // let events: Pin<&mut Stream> = events;
        while let Some(item) = events.next().await {
            if let Event::ReplayBufferSaved { path } = item {
                println!("{}", path.display());
                app.emit("replay_buffer", path).unwrap();
            }
            // if let Event::ReplayBufferSaved(event) = item {

            // }
            // if matches!(item,Event::ReplayBufferSaved){
            //     app.emit("replay_buffer", item.)
            // }
        }
        Ok(())
    });

    Ok(())
}

#[tauri::command]
async fn obs_save_replay_buffer(state: State<'_, Mutex<obs::ObsClass>>) -> Result<String, String> {
    let state = state.lock().await;
    state.save_replay_buffer().await
}
#[tauri::command]
fn detect_cameras() {
    let path = Path::new("./bin/opencv.exe");
    println!("path: {}", path.display());
    // Command::new(path).arg("detect_cameras").spawn().expect("failed to run");
    Command::new(path)
        .arg("detect_cameras")
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .expect("failed to run");
}
#[tauri::command]
fn preview_camera(index: i32) {
    let path = Path::new("./bin/opencv.exe");
    // Command::new(path).arg("preview_camera")/*.creation_flags(CREATE_NO_WINDOW)*/.arg(index.to_string()).spawn().expect("failed to run");
    Command::new(path)
        .arg("preview_camera")
        .creation_flags(CREATE_NO_WINDOW)
        .arg(index.to_string())
        .spawn()
        .expect("failed to run");
}

#[tauri::command]
async fn opencv_watch(index: i32) {
    let path = Path::new("./bin/opencv.exe");
    Command::new(path)
        .arg("watch") /* .creation_flags(CREATE_NO_WINDOW)*/
        .arg(index.to_string())
        .spawn()
        .expect("failed to run");
    // Command::new(path).arg("watch").creation_flags(CREATE_NO_WINDOW).arg(index.to_string()).spawn().expect("failed to run");
}

#[tauri::command]
async fn set_camera_index(
    state: State<'_, Mutex<CameraIndexState>>,
    index: i32,
) -> Result<i32, ()> {
    let mut state = state.lock().await;
    state.index = index;
    Ok(state.index)
}
#[tauri::command]
async fn get_camera_index(state: State<'_, Mutex<CameraIndexState>>) -> Result<i32, ()> {
    let state = state.lock().await;
    Ok(state.index)
}

#[derive(Default)]
struct CameraIndexState {
    index: i32,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                window.open_devtools();
            }
            app.manage(Mutex::new(obs::ObsClass::new()));
            app.manage(Mutex::new(CameraIndexState::default()));
            let app_handle = app.app_handle().clone();
            thread::spawn(|| socket::start_server(app_handle));
            // app.manage(Mutex::new(camera::CameraClass::new()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_log::Builder::new().target(tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Folder { path: std::path::PathBuf::from("./logs"),file_name: None })).build())
        .invoke_handler(tauri::generate_handler![
            greet,
            obs_login,
            obs_start,
            obs_event_listen,
            obs_save_replay_buffer,
            detect_cameras,
            preview_camera,
            opencv_watch,
            get_camera_index,
            set_camera_index,
            delete_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
