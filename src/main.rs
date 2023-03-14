mod render;
mod input;

use render::render;
use sdl_degreeproject::constvalues;
use sdl_degreeproject::datatypes::vector::Vector2;
use sdl_degreeproject::networking::{server, client};

use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::sync::{Mutex, Arc};
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::{Event};
use sdl2::keyboard::{Keycode};
use sdl2::rect::{Rect, Point};
use sdl2::image::{self,LoadTexture,InitFlag};

#[derive(Debug)]
pub struct Player
{
    position: Point,
    sprite: Rect,
    speed: i32
}

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 800;

// fn printnetworkdata(buffer: &[u8]) {
//     println!("Received mainthread data: {}", String::from_utf8_lossy(buffer));
// }

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let _serverarg = String::from("server");
    let dedserverarg = String::from("dedserver");

    let sharedbuffer = Arc::new(Mutex::new([0; constvalues::BUF_SIZE]));
    
    if args.contains(&dedserverarg)
    {
        server::createlan();
        loop {
            
        }
    }
    
    let netclient = client::init();
    // server::createlan();

    let ownedbuf = sharedbuffer.to_owned();
    netclient.recieve(move|netbuffer: &mut [u8]| {
        let clonedbuf = ownedbuf.clone();
        let mut locbuffer = clonedbuf.lock().unwrap();
        locbuffer.copy_from_slice(netbuffer);
    });

    //We know network positions
    let networkplayerpositions: HashMap<u8, Point> = HashMap::new();

    //Local player positions
    let mut playerpositions: HashMap<u8, Point> = HashMap::new();

    // for e in networkplayerpositions {
    //     playerpositions.insert(e.0, e.1);
    // }

    // networkplayerpositions.remove(netclient.id);

    println!("Connected to localhost");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::PNG).expect("askldj");

    let window = video_subsystem.window("My new SDL Window", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();


    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("face.png")?;

    let mut player = Player { 
        position: Point::new(0, 0), 
        sprite: Rect::new(0,0,32,32), 
        speed: 5 
    };

    let mut players: Vec<Player> = Vec::new();

    canvas.set_draw_color(Color::RGB(0, 255, 255));

    let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_owned();
    path.push("fontaa.ttf");

    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font(path,32).unwrap();

    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let texture_creator = canvas.texture_creator();
    
    let text_surface = font.render("Player{}").blended(Color::RGBA(255,0,0,255)).unwrap();
    let text_texture = texture_creator.create_texture_from_surface(&text_surface).unwrap();
   

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut left_is_held_down = false;
    let mut right_is_held_down = false;
    let mut up_is_held_down = false;
    let mut down_is_held_down = false;

    //const _GRID_SIZE:u32 = (SCREEN_HEIGHT / 10) * (SCREEN_WIDTH / 10);

    const GRID_WIDTH: u32 = SCREEN_WIDTH / 10;
    const GRID_HEIGHT: u32 = SCREEN_HEIGHT / 10;

    const ARR_SIZE: u32 = GRID_HEIGHT * GRID_WIDTH;

    let mut _grid: [Rect;ARR_SIZE as usize] = [Rect::new(1, 1, 1, 1); ARR_SIZE as usize];

    
    let rows =  GRID_HEIGHT;
    let columns  =  GRID_WIDTH;


    for i in 0..(rows * columns) {
        let row = i / columns;
        let col = i % columns;

        let rect = Rect::new(100 * row as i32, 100 * col as i32, 100, 100);
        
        _grid[i as usize] = rect;
        
        
    }

    // Number of frames to average over
    let num_frames = 60;
    // Array to store frame times
    let mut frame_times: [u32; 60] = [0; 60];
    // Index for frame times array
    let mut frame_index = 0;

    let mut i = 0;

    let mut prevplayerpos = player.position;

    'running: loop {

        let start_time = unsafe { sdl2::sys::SDL_GetTicks() };

        i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255-i));
        

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {break 'running},

                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {left_is_held_down = true;}
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {right_is_held_down = true;}
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {up_is_held_down = true;}
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {down_is_held_down = true;}

                Event::KeyUp { keycode: Some(Keycode::Left), .. } => { left_is_held_down = false;}
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => { right_is_held_down = false;}
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => { up_is_held_down = false;}
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => { down_is_held_down = false;}
                _ => {}
            }
        }

        

        if left_is_held_down {
            player.position = player.position.offset(-player.speed, 0);
            // playerpositions.get_mut(netclient.id) = player.position.offset(x, y);
        } 
        if right_is_held_down {
            player.position = player.position.offset(player.speed, 0);
        } 
        if up_is_held_down {
            player.position = player.position.offset(0, -player.speed);
        }
        if down_is_held_down {
            player.position = player.position.offset(0, player.speed);
        }

        if player.position != prevplayerpos {
            netclient.sendpos(Vector2 {x: player.position.x(), y: player.position.y()});
            prevplayerpos = player.position;
        }

        let (canvas_width, canvas_height) = canvas.output_size()?;

        let screen_position = player.position + Point::new(canvas_width as i32/2, (canvas_height as i32 / 2) - 32) ;
        let screen_rect = Rect::from_center(screen_position, text_texture.query().width, text_texture.query().height);



        canvas.clear();

        // for _rect in _grid.iter() {

        //     let _x = event_pump.mouse_state().x();
        //     let _y = event_pump.mouse_state().y();

        //     if _x < (_rect.x + _rect.width() as i32) &&
        //         _x > _rect.x &&
        //         _y > _rect.y + _rect.height() as i32 &&
        //         _y < _rect.y
            
        //     {
                
        //     };

        //     canvas.set_draw_color(Color::RGB(255, 0, 0));
        //     canvas.draw_rect(*_rect).unwrap();
        // }


        let sbuffer = sharedbuffer.clone();
        let bufslice = sbuffer.lock().unwrap();
        let _info = bufslice.as_slice();
        if _info[0] == 17 {
            let pcount = (_info.len() - 1) / 2;
            while players.len() < pcount {
                players.push(Player {
                    position: Point::new(1, 1), 
                    sprite: Rect::new(0,0,32,32), 
                    speed: 5 
                })
            }
            for i in 0..pcount {
                println!("Render player {} with position", i);
            }
        }
        
        
        
        render(&mut canvas, Color::RGB(30, 30, 30), &texture,&player).unwrap();
        canvas.copy(&text_texture, None, screen_rect).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));


        // Get time after rendering frame
        let end_time = unsafe { sdl2::sys::SDL_GetTicks() };
        
        // Calculate time taken to render frame
        let frame_time = end_time - start_time;
        
        // Store frame time in array and increment index
        frame_times[frame_index] = frame_time;
        frame_index += 1;

        if frame_index >= num_frames {
            // Calculate average FPS over num_frames frames
            let total_frame_time: u32 = frame_times.iter().sum();
            let avg_frame_time: f64 = (total_frame_time as f64) / (num_frames as f64);
            let avg_fps: f64 = 1000.0 / avg_frame_time;

            println!("Average FPS: {}", avg_fps);

            // Reset index and clear array
            frame_index = 0;
            for i in &mut frame_times {
                *i=0;
            }
       }
    }

    Ok(())
    // ...
}