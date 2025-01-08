use std::io::Error;
use std::net::{TcpListener, TcpStream};
use std::{
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddr},
    thread,
};
use tauri::{AppHandle, Emitter, Manager};

fn send_webview(app: &AppHandle, message: String) {
    app.emit("opencv", message).unwrap();
}

fn send_recv(app: AppHandle, mut stream: TcpStream) -> Result<(), Error> {
    println!("Connection from: {}", stream.peer_addr()?);
    let mut buf = [0; 512];
    loop {
        let byte_read = stream.read(&mut buf)?;
        if byte_read == 0 {
            return Ok(());
        }
        let message = String::from_utf8(buf[..byte_read].to_vec()).expect("");
        println!("{:?}", message);
        send_webview(&app, message);
        // stream.write(&buf[..byte_read])?;
    }
}

pub fn start_server(app: AppHandle) {
    // let port = 3232;
    let listener = TcpListener::bind("127.0.0.1:3232").expect("Could not bind");
    println!("Server Started");
    for stream in listener.incoming() {
        match stream {
            Err(e) => {
                eprintln!("failed: {}", e)
            }
            Ok(stream) => {
                let _app = app.app_handle().clone();
                thread::spawn(move || {
                    send_recv(_app, stream).unwrap_or_else(|e| eprintln!("{:?}", e));
                });
            }
        }
    }
}
