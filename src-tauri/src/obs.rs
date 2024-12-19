use obws::Client;
use tokio::sync::Mutex;
use std::sync::Arc;
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
    pub async fn set_virtual_cam(&self,) -> Result<String,String>{
        let client = self.get_client().await?;
        let res = client.virtual_cam().start().await;
        if let Err(e) = res {
            Err(e.to_string())
        }else{
            Ok("turn up virtual cam".to_string())
        }
    }
}

