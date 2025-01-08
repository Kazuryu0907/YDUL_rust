use futures_util::{pin_mut, Stream, StreamExt};
use obws::{events::Event, Client};
use std::{mem::discriminant, path::PathBuf, pin::Pin, sync::Arc, time::Duration};
use tauri::{AppHandle, Emitter};
use tokio::time::sleep;
// use anyhow::Result;

// static OBS_CLIENT: Mutex<Option<Client>> = Mutex::new(None);

pub struct ObsClass {
    pub client: Option<Arc<Client>>,
}

impl ObsClass {
    pub fn new() -> Self {
        ObsClass { client: None }
    }
    pub async fn get_client(&self) -> Result<Arc<Client>, String> {
        match &self.client {
            Some(client) => Ok(Arc::clone(client)),
            None => Err("Not Connected to OBS".to_string()),
        }
    }

    pub async fn login(
        &mut self,
        host: &str,
        port: u16,
        password: Option<&str>,
    ) -> Result<String, String> {
        let client = Client::connect(host, port, password).await;
        match client {
            Ok(client) => {
                self.client = Some(Arc::new(client));
                Ok("login to OBS".to_string())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    pub async fn set_virtual_cam(&self) -> Result<String, String> {
        let client = self.get_client().await?;
        let status = match client.virtual_cam().status().await {
            Ok(status) => status,
            Err(e) => return Err(e.to_string()),
        };
        if status == true {
            return Ok("already up virtual cam".to_string());
        }
        let res = client.virtual_cam().start().await;
        match res {
            Err(e) => Err(e.to_string()),
            Ok(_) => Ok("turn up virtual cam".to_string()),
        }
    }

    pub async fn set_replay_buffer(&self) -> Result<String, String> {
        let client = self.get_client().await?;
        let status = match client.replay_buffer().status().await {
            Ok(status) => status,
            Err(e) => return Err(e.to_string()),
        };
        if status == true {
            return Ok("already up replay buffer".to_string());
        }
        let res = client.replay_buffer().start().await;
        match res {
            Ok(_) => Ok("turn up replay buffer".to_string()),
            Err(e) => Err(e.to_string()),
        }
    }

    pub async fn save_replay_buffer(&self) -> Result<String, String> {
        let client = self.get_client().await?;
        let status = client.replay_buffer().save().await;
        if let Err(e) = status {
            return Err(e.to_string());
        }
        // sleep(Duration::from_millis(10)).await;
        // let last_replay = client.replay_buffer().last_replay().await;
        // match last_replay{
        //     Ok(path) => Ok(path),
        //     Err(e) => Err(e.to_string())
        // }
        Ok("success".to_string())
    }

    pub async fn events(&self, app: AppHandle) -> Result<String, String> {
        let client = self.get_client().await?;
        let events = match client.events() {
            Ok(stream) => stream,
            Err(e) => return Err(e.to_string()),
        };
        pin_mut!(events);
        // let events: Pin<&mut Stream> = events;
        while let Some(item) = events.next().await {
            println!("{:?}", item);
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
        Ok("".to_string())
    }
}
