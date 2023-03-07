use std::env;

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
    let sdlarg = String::from("showsdl");
    
    
    if args.contains(&serverarg){
        server::createlan();
    }
    client::connect().expect("Couldnt connect to server!");
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
    
}