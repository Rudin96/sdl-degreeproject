mod render;
mod player;
mod objects;

use player::player_module;
use sdl_degreeproject::constvalues;
use sdl_degreeproject::datatypes::vector::Vector2;
use sdl_degreeproject::networking::{client, server};
use self::objects::object_module::Objects;
use self::player::player_module::Player;
use self::render::render_text;

use render::render_players;

use objects::object_module::place_furniture;

use sdl2::mouse::{MouseButton};
use sdl2::render::{TextureCreator, Texture};
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{env, vec};
use std::path::Path;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect, Point};
use sdl2::image::{self,LoadTexture,InitFlag};
pub struct Tile
{
    rect: Rect,
    occupied: bool,
    furniture: Option<Objects>,
    position: Point,
    highlight: bool
}

const SCREEN_WIDTH: u32 = 1000;
const SCREEN_HEIGHT: u32 = 1000;

const GRID_WIDTH: i32 = (SCREEN_WIDTH / 10) as i32;
const GRID_HEIGHT: i32 = (SCREEN_HEIGHT / 10) as i32;

pub(crate) fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let sdl_context = sdl2::init()?;
    
    //Read network info(clientid, position) to shared buffer
    if args.contains(&String::from("server"))
    {
        server::createlan();
    }
    let sharedbuffer = Arc::new(Mutex::new(Vec::<Vec::<u8>>::new()));
    let netbff = sharedbuffer.clone();
    let netclient = client::init();

    netclient.recieve(move |netbuffer| {
        
        let mut buffer = netbff.lock().unwrap();
        buffer.push(netbuffer.to_vec());
    });

    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::PNG).unwrap();

    let window = video_subsystem.window("My new SDL Window", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_owned();
    path.push("fontaa.ttf");
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font(path,32).unwrap();
    font.set_style(sdl2::ttf::FontStyle::NORMAL);
    

    let texture = texture_creator.load_texture("face.png")?;
    let catman = texture_creator.load_texture("catman.png")?;


    let mut player = Player
    {
        position: Point::new(0, 0), 
        sprite: Rect::new(0,0,32,32), 
        speed: 5,
        player_texture: texture,
        player_id: 0,
        text_texture: None    
    };


    let mut player_input = player_module::PlayerInput::default();

    let mut mouse_position = Point::new(0,0);


    canvas.set_draw_color(Color::RGB(0, 255, 255));

    let mut event_pump = sdl_context.event_pump().unwrap();
    
    let rows =  GRID_HEIGHT;
    let columns  =  GRID_WIDTH;

    let mut tile_rect = Rect::new(0,0,0,0);
    let mut sprite_rect = Rect::new(0,0,0,0);


    let mut _new_grid: Vec<Tile> = vec![];
    create_grid(&rows, &columns, &mut _new_grid);


    // Number of frames to average over
    let num_frames = 60;
    // Array to store frame times
    let mut frame_times: [u32; 60] = [0; 60];
    // Index for frame times array
    let mut frame_index = 0;

    let mut i = 0;

    let mut prevPlayerPos = player.position;

    let mut playerpositions: HashMap<u8, Vector2> = HashMap::new();

    'running: loop {

        let start_time = unsafe { sdl2::sys::SDL_GetTicks() };


        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255-i));
        

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {break 'running},

                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {player_input.m1_is_down = true}
                Event::MouseButtonUp { mouse_btn: MouseButton::Left, .. } => {player_input.m1_is_down = false}
                
                Event::MouseButtonDown { mouse_btn: MouseButton::Right, .. } => {player_input.m2_is_down = true}
                Event::MouseButtonUp { mouse_btn: MouseButton::Right, .. } => {player_input.m2_is_down = false}

                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {player_input.left_is_held_down = true;}
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {player_input.right_is_held_down = true;}
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {player_input.up_is_held_down = true;}
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {player_input.down_is_held_down = true;}

                Event::KeyUp { keycode: Some(Keycode::Left), .. } => { player_input.left_is_held_down = false;}
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => { player_input.right_is_held_down = false;}
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => { player_input.up_is_held_down = false;}
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => { player_input.down_is_held_down = false;}
                _ => {}
            }
        }


        if player_input.left_is_held_down {
            player.position = player.position.offset(-player.speed, 0);
        } 
        if player_input.right_is_held_down {
            player.position = player.position.offset(player.speed, 0);
        } 
        if player_input.up_is_held_down {
            player.position = player.position.offset(0, -player.speed);
        }
        if player_input.down_is_held_down {
            player.position = player.position.offset(0, player.speed);
        }

        mouse_position = (event_pump.mouse_state().x(),event_pump.mouse_state().y()).into();


        
        //Send local position to server
        if player.position != prevPlayerPos {
            netclient.sendpos(Vector2 {x: player.position.x, y: player.position.y});
            prevPlayerPos = player.position;
        }

        
        canvas.clear();

        //Here we deserialize to playerposition hashmap
        {
            let vec = sharedbuffer.lock().unwrap();
            for x in vec.to_vec() {
                let positions_des = String::from_utf8(x).unwrap();
                // println!("Bytes are: {:?}", positions_des);
                playerpositions = serde_json::from_str(&positions_des).unwrap();
            }
        }

        //Here we read everything from it
        // for p in &playerpositions {
        //     println!("Client {} has pos {:?}", p.0, p.1);
        // }

        sharedbuffer.lock().unwrap().clear();

        check_tile(&mut _new_grid, &mouse_position, &player_input);
        draw_tiles(&mut _new_grid, &mut canvas,&mut sprite_rect, &mut tile_rect);

        //check_tile(&mut _new_grid, &event_pump, &mut canvas, &player_input, &mut sprite_rect, &mut tile_rect);
        
        
        //Draw objects
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        for piece in &_new_grid {

            match &piece.furniture {
                Some(furniture) => canvas.copy(&catman,furniture.sprite,furniture.rect).unwrap(),
                _ => ()
            }
        }


        for playerclient in &playerpositions {

            render_players(Color::RGB(i, 64, 255 - i),&mut canvas,playerclient,&player);

            let player_text = "Player ".to_owned() + &playerclient.0.to_string();
            let text_surface = font.render(&player_text.to_string()).blended(Color::RGBA(255,0,0,255)).unwrap();
            let binding = canvas.texture_creator();
            let text_texture = binding.create_texture_from_surface(&text_surface).unwrap();

            render_text(&mut canvas, &text_texture, playerclient).unwrap();

            //render_player(&mut canvas, Color::RGB(i, 64, 255 - i),&player,&font).unwrap();
        }

        
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


fn draw_tiles(_new_grid: &mut Vec<Tile>,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,sprite_rect: &mut Rect,tile_rect: &mut Rect) {
    
    for tile in  _new_grid{
        if tile.highlight {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.fill_rect(tile.rect).unwrap();
        }

        else if !tile.highlight {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.draw_rect(tile.rect).unwrap();  
        }

        if tile.occupied {
            place_furniture(canvas, tile, sprite_rect, tile_rect)
        }
    }
}

// fn check_tile(_new_grid: &mut Vec<Tile>, 
//     event_pump: &sdl2::EventPump, 
//     canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, 
//     player_input: &player_module::PlayerInput, 
//     sprite_rect: &mut Rect, 
//     tile_rect: &mut Rect) 
//     {
//     for tile in  _new_grid{
//         let _x = event_pump.mouse_state().x();
//         let _y = event_pump.mouse_state().y();

//         if  _x < (tile.rect.x + tile.rect.width() as i32) &&
//             _x > tile.rect.x &&
//             _y < (tile.rect.y + tile.rect.height() as i32) &&
//             _y > tile.rect.y
//             {
//                 canvas.set_draw_color(Color::RGB(0, 255, 0));
//                 canvas.draw_rect(tile.rect).unwrap();

//                 if player_input.m1_is_down && !tile.occupied{

//                     place_furniture(canvas, tile, sprite_rect, tile_rect);

//                 }

//                 if player_input.m2_is_down && tile.occupied{

//                     tile.occupied = false;
//                     tile.furniture = None;

                
    
//                     //Get the rect location
//                     //Draw a new rect based on the location
//                     //Make sure its drawn in the middle of the current rect
//                     //postition + width to make sure its properly placed
//                     //Tag current one as occupied?

//                 }
//             }
//         else {
//             canvas.set_draw_color(Color::RGB(255, 0, 0));
//             canvas.draw_rect(tile.rect).unwrap();
//         }
//     }
// }

fn check_tile(_new_grid: &mut Vec<Tile>,
    mouse_position: &Point, 
    player_input: &player_module::PlayerInput) 
    {
    for tile in  _new_grid{
        
        if (mouse_position.x / 100) == tile.position.x && (mouse_position.y / 100) == tile.position.y {
                tile.highlight = true;

                if player_input.m1_is_down && !tile.occupied{
                    tile.occupied = true;
                }

                if player_input.m2_is_down && tile.occupied{

                    tile.occupied = false;
                    tile.furniture = None;
                }
            }
        else {
                tile.highlight = false;
            }
        
    }

}




fn create_grid(rows: &i32, columns: &i32, _new_grid: &mut Vec<Tile>) {
    for i in 0..(rows * columns) {
    
        let row = i / columns;
        let col = i % columns;

        let tile = Rect::new(100 * row as i32, 100 * col as i32, 100, 100);

        let new_tile = Tile {
            rect: tile,
            occupied: false,
            furniture: None,
            position: Point::new(row,col),
            highlight: false
        };
        _new_grid.push(new_tile)
    }
}


