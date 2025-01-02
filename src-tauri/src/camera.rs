use opencv::{
    prelude::*,
    videoio,
    highgui
};
use anyhow::Result;

pub fn show_camera(index: i32) -> Result<()> {
    highgui::named_window("window",highgui::WINDOW_FULLSCREEN);
    let mut cam = videoio::VideoCapture::new(index,videoio::CAP_ANY)?;
    let mut frame = Mat::default();
    loop {
        cam.read(&mut frame)?;
        highgui::imshow("window", &frame);
        let key = highgui::wait_key(1)?;
        if key == 113{break;}
    }
    Ok(())
}