mod render;
mod player;
mod objects;

use nalgebra::Vector2;

use player::player_module;
use sdl2::libc::_IOFBF;
use sdl2::sys::{SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency, SDL_GetTicks, SDL_FPoint};
use sdl_degreeproject::datatypes::vector::Custom_Vector2;
use sdl_degreeproject::networking::{client, server};
use self::objects::object_module::{Objects, allocate_object};
use self::player::player_module::Player;
use self::render::render_text;

use render::render_players;

use sdl2::mouse::{MouseButton};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{env};
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
    highlight: bool,
    imageid: u32
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
    let mut netclient = client::init();

    if args.contains(&String::from("connect")) {
        let res = args.binary_search(&String::from("connect")).unwrap();
        netclient.connect(args.get(res).unwrap().to_string());
    } else {
        netclient.connect("127.0.0.1".to_string());
    }

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

    let mut path = std::env::current_dir().unwrap().to_owned();
    
    path.push("fontaa.ttf");
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font(path,32).unwrap();
    font.set_style(sdl2::ttf::FontStyle::NORMAL);
    

    let texture = texture_creator.load_texture("face.png")?;
    let catman = texture_creator.load_texture("catman.png")?;
    let players = texture_creator.load_texture("characters.png")?;


    let mut img_hash: HashMap<u8,Rect> = HashMap::new();
    create_player_images(&players, &mut img_hash);

    let mut player = Player
    {
        position: Point::new(0, 0), 
        sprite: Rect::new(0,0,16,32), 
        speed: 0.5,
        player_texture: players,
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


    let mut grid_map: HashMap<(i32,i32),Tile> = HashMap::new();
    
    create_hash_grid(&rows, &columns, &mut grid_map);

   

    
    let mut prevPlayerPos = player.position;

    let mut playerpositions: HashMap<u8, Custom_Vector2> = HashMap::new();

    const FRAME_VALUES: usize = 10;
    let mut frame_times: [u32; FRAME_VALUES] = [0; FRAME_VALUES];
    let mut frame_time_last: u32 = unsafe { SDL_GetTicks() };
    let mut frame_count: usize = 0;


    'running: loop {


        let current_ticks = unsafe { SDL_GetTicks() };

        frame_times[frame_count % FRAME_VALUES] = current_ticks - frame_time_last;

        frame_time_last = current_ticks;
        
        let count: usize;
        if frame_count < FRAME_VALUES {
            count = frame_count;
        }    
        else {
        count = FRAME_VALUES;
        } 
        frame_count += 1;

        let mut frame_time_average: f32 = 0.0;

        for i in 0..count {
            frame_time_average += frame_times[i] as f32;
        }
        frame_time_average /= count as f32;




        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255-i));
        canvas.set_draw_color(Color::RGB(50,50, 50));
        

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


        let mut a = Vector2::new(0.0,0.0);
        a = a.normalize();

        check_player_input(&player_input, &mut a);

        a = a.normalize();
        
        player.position = player.position.offset((-player.speed * a.x * frame_time_average) as i32,(-player.speed * a.y * frame_time_average) as i32);



        
        mouse_position = (event_pump.mouse_state().x(),event_pump.mouse_state().y()).into();


        
        //Send local position to server
        if player.position != prevPlayerPos {
            netclient.sendpos(Custom_Vector2 {x: player.position.x, y: player.position.y});
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

        
        sharedbuffer.lock().unwrap().clear();



        //Allocate and draw grid
        check_hash_tile(&mut grid_map, &mouse_position, &player_input);
        draw_tile_grid(&mut grid_map, &mut canvas, &mut tile_rect);

        //Draw objects
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        for piece in &grid_map {

            if piece.1.furniture.is_some() && piece.1.occupied {

                if piece.1.imageid == 0 {
                    match &piece.1.furniture {
                        Some(furniture) => canvas.copy(&catman,furniture.sprite,furniture.rect).unwrap(),
                        _ => () 
                        }
                    } 
                }
        }


        for playerclient in &playerpositions {

            player.sprite = img_hash[playerclient.0];

            render_players(Color::RED,&mut canvas,playerclient,&mut player);

            let player_text = "Player ".to_owned() + &playerclient.0.to_string();
            let text_surface = font.render(&player_text.to_string()).blended(Color::RGBA(255,0,0,255)).unwrap();
            let binding = canvas.texture_creator();
            let text_texture = binding.create_texture_from_surface(&text_surface).unwrap();

            render_text(&mut canvas, &text_texture, playerclient).unwrap();

            //render_player(&mut canvas, Color::RGB(i, 64, 255 - i),&player,&font).unwrap();
        }

        
        canvas.present();

        
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        
    }

    Ok(())
    // ...
}

fn check_player_input(player_input: &player_module::PlayerInput, a: &mut nalgebra::Matrix<f32, nalgebra::Const<2>, nalgebra::Const<1>, nalgebra::ArrayStorage<f32, 2, 1>>) {
    match ( player_input.left_is_held_down,
            player_input.right_is_held_down,
            player_input.down_is_held_down,
            player_input.up_is_held_down) {
    
        (true,false,true,false) => {//Left Down
            a.x = 1.0;
            a.y = -1.0;
        }
        (true,false,false,true) => {//Left Up
            a.x = 1.0;
            a.y = 1.0;
        }
        (false,true,false,true) => {//Right Up
            a.x = -1.0;
            a.y = 1.0;
        }
        (false,true,true,false) => {//Right Down
            a.x = -1.0;
            a.y = -1.0;
        }
        (false,false,true,false) => {//Up
            a.x = 0.0;
            a.y = -1.0;
        }
        (false,false,false,true) => {//Down
            a.x = 0.0;
            a.y = 1.0;
        }
        (true,false,false,false) => {//Left
            a.x = 1.0;
            a.y = 0.0;
        }
        (false,true,false,false) => {//Right
            a.x = -1.0;
            a.y = 0.0;
        }
        _ => {}
    }
}

fn create_player_images(players: &sdl2::render::Texture, img_hash: &mut HashMap<u8, Rect>) {
    for i in 0..players.query().height / 64 {
    
        let img_src = Rect::new(0,32*i as i32 ,16,32);

        img_hash.insert(i as u8, img_src);
    }
}

fn draw_tile_grid(_tile_map: &mut HashMap<(i32,i32),Tile>,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,tile_rect: &mut Rect) {
    
    for tile in  _tile_map{
        if tile.1.highlight {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.fill_rect(tile.1.rect).unwrap();
        }

        else if !tile.1.highlight {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.draw_rect(tile.1.rect).unwrap();  
        }
    }
}

fn check_hash_tile(_tile_map: &mut HashMap<(i32,i32),Tile>,
    mouse_position: &Point, 
    player_input: &player_module::PlayerInput) 
    {

    let key = &(mouse_position.x / 100,mouse_position.y/ 100);
    let mut keys_to_modify = Vec::new();


    for tile in _tile_map.into_iter() {

        if tile.1.position.x == key.0 && tile.1.position.y == key.1{
            tile.1.highlight = true;
            
            if player_input.m1_is_down && !tile.1.occupied{

                allocate_object(tile.1);

                if let Some(a) = &tile.1.furniture {
                    
                    for i  in -a.object_width+1 ..  a.object_width {
                        keys_to_modify.push((key.0+i,key.1));
                    }   
                }
            }
            if player_input.m2_is_down && tile.1.occupied{

                tile.1.occupied = false;
                tile.1.furniture = None;
            }
                   
        }
        else {
        tile.1.highlight = false;    
        }

    }

    for tile in keys_to_modify {
        if let Some(a) = _tile_map.get_mut(&tile) {
            a.occupied = true;
        }
    }


}



fn create_hash_grid(rows: &i32, columns: &i32, _tile_map: &mut HashMap<(i32,i32),Tile>) {
    for i in 0..(rows * columns) {
    
        let row = i / columns;
        let col = i % columns;

        let tile = Rect::new(100 * row as i32, 100 * col as i32, 100, 100);

        let new_tile = Tile {
            rect: tile,
            occupied: false,
            furniture: None,
            position: Point::new(row,col),
            highlight: false,
            imageid: 0
        };
        _tile_map.insert((row,col),new_tile);
    }
}



