use std::env;
use std::sync::Arc;
use std::sync::Mutex;

use sdl_degreeproject::networking::client;
use sdl_degreeproject::networking::server;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let serverarg = String::from("server");
    let sdlarg = String::from("window");
    let noclientarg = String::from("noclient");
    
    let sharedbuffer = Arc::new(Mutex::new([0; 1024]));
    
    if args.contains(&serverarg){
        server::createlan();
        let bufferclone = sharedbuffer.clone();
        if !args.contains(&noclientarg)
        {
            client::connect("127.0.0.1", move |received_bytes| {
                let mut x = bufferclone.lock().unwrap();
                x.copy_from_slice(received_bytes);
                // println!("Received on main thread: {}", String::from_utf8_lossy(x.as_slice()));
            }).expect("Couldnt connect to server!");
        }
        loop {
            
        }
    }else {
        let bufferclone = sharedbuffer.clone();
        client::connect("10.24.1.182", move |received_bytes| {
            let mut x = bufferclone.lock().unwrap();
            x.copy_from_slice(received_bytes);
            // println!("Received on main thread: {}", String::from_utf8_lossy(x.as_slice()));
        }).expect("Couldnt connect to server!");
        loop {
            let recbuffer = sharedbuffer.clone();
            let recbuffer2 = recbuffer.lock().unwrap();
            println!("main thread netinfo is: {}", String::from_utf8_lossy(recbuffer2.as_slice()));
        }
    }
    // manager::senddata("Test message");
    
    if args.contains(&sdlarg)
    {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("My new SDL Window", 800, 600)
            .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
            .build()
            .unwrap();
        let mut _canvas = window.into_canvas()
            .index(find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

        loop {
            _canvas.present();
        }
    }
    
    // loop {
    //     let recbuffer = sharedbuffer.clone();
    //     let recbuffer2 = recbuffer.lock().unwrap();
    //     println!("main thread netinfo is: {}", String::from_utf8_lossy(recbuffer2.as_slice()));
    // }
    
}