use obws::Client;
use tokio::sync::Mutex;
use std::sync::Arc;
use std::error::Error;
// use anyhow::Result;

// static OBS_CLIENT: Mutex<Option<Client>> = Mutex::new(None);

pub struct ObsClass{
    pub client: Option<Arc<Client>>
}

impl ObsClass{
    pub fn new() -> Self{
        ObsClass {client: None}
    }
    async fn get_client(&self) -> Result<Arc<Client>,String> {
        match &self.client {
            Some(client) => Ok(Arc::clone(client)),
            None => Err("Not Connected to OBS".to_string())
        }
    }

    pub async fn login(&mut self, host:&str,port:u16,password:Option<&str>) -> Result<String,String>{
        let client = Client::connect(host, port, password).await;
        match client {
            Ok(client) => {
                self.client = Some(Arc::new(client));
                Ok("logined to OBS".to_string())
            },
            Err(e) => Err(e.to_string())
        }
    }
    pub async fn set_virtual_cam(&self) -> Result<String,String>{
        let client = self.get_client().await?;
        let status = match client.virtual_cam().status().await {
            Ok(status) => status,
            Err(e) => return Err(e.to_string())
        };
        if status == true { return Ok("already up virtual cam".to_string()) }
        let res = client.virtual_cam().start().await;
        match res{
            Err(e) => Err(e.to_string()),
            Ok(_) => Ok("turn up virtual cam".to_string())
        }
    }

    pub async fn set_replay_buffer(&self) -> Result<String,String>{
        let client = self.get_client().await?;
        let status = match client.replay_buffer().status().await {
            Ok(status) => status,
            Err(e) => return Err(e.to_string())
        };
        if status == true { return Ok("already up replay buffer".to_string())}
        let res = client.replay_buffer().start().await;
        match res {
            Ok(_) => Ok("turn up replay buffer".to_string()),
            Err(e) => Err(e.to_string())
        }
    }
}

