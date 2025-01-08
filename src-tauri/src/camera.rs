// use tokio::sync::Mutex;

// use opencv::{
//     prelude::*,
//     videoio,
//     highgui
// };
use opencv;

pub fn a(){

}
// use anyhow::Result;

// pub struct CameraClass{
//     stop_signal: Mutex<bool>,
// }

// impl CameraClass{
//     pub fn new() -> Self{
//         CameraClass {stop_signal: Mutex::new(false)}
//     }
//     pub async fn show_camera(&self,index: i32) -> Result<()> {
//         let _ = highgui::named_window("window",highgui::WINDOW_FULLSCREEN);
//         let mut cam = videoio::VideoCapture::new(index,videoio::CAP_ANY)?;
//         let mut frame = Mat::default();
//         while !*self.stop_signal.lock().await {
//             cam.read(&mut frame)?;
//             let _ = highgui::imshow("window", &frame);
//             let key = highgui::wait_key(1)?;
//             if key == 113{break;}
//         }
//         Ok(())
//     }

//     pub async fn set_stop_signal(&mut self, signal:bool){
//         // signal更新
//         *self.stop_signal.lock().await = signal;
//     }
// }

// pub fn show_camera(index: i32) -> Result<()> {
//     let _ = highgui::named_window("window",highgui::WINDOW_FULLSCREEN);
//     let mut cam = videoio::VideoCapture::new(index,videoio::CAP_ANY)?;
//     let mut frame = Mat::default();
//     loop {
//         cam.read(&mut frame)?;
//         let _ = highgui::imshow("window", &frame);
//         let key = highgui::wait_key(1)?;
//         if key == 113{break;}
//     }
//     Ok(())
// }