mod render;
mod player;
mod objects;

//External Crates
use nalgebra::Vector2;
use rand::Rng;
use noise::{NoiseFn, Perlin};




//SDL and custom crates
use player::player_module;

use sdl2::render::Texture;
use sdl2::sys::{SDL_GetTicks, SDL_GetPerformanceCounter, SDL_GetPerformanceFrequency};
use sdl_degreeproject::datatypes::vector::Custom_Vector2;
use sdl_degreeproject::networking::{client, server};
use self::objects::object_module::{Objects, allocate_object};
use self::player::player_module::Player;
use self::render::render_text;

use render::render_players;

use sdl2::mouse::{MouseButton};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, thread};
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

#[derive(Default, Debug, Clone, Copy)]
pub struct Test {
    x: i32,
    y: i32,
    z: i32,

    a: f64,
    b: f64
}

const SCREEN_WIDTH: u32 = 1600;
const SCREEN_HEIGHT: u32 = 900;

const GRID_WIDTH: i32 = (SCREEN_WIDTH / 10) as i32;
const GRID_HEIGHT: i32 = (SCREEN_HEIGHT / 10) as i32;

pub(crate) fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let sdl_context = sdl2::init()?;
    
    if args.contains(&String::from("dserver"))
    {
        server::createlan();
        loop {
            
        }
    }

    //Read network info(clientid, position) to shared buffer
    if args.contains(&String::from("server"))
    {
        thread::spawn(|| {
            server::createlan();
            // loop {
                
            // }
        });
    }
    let sharedbuffer = Arc::new(Mutex::new(Vec::<Vec::<u8>>::new()));
    let netbff = sharedbuffer.clone();
    let mut netclient = client::init();

    if args.contains(&String::from("connect")) {
        let res = args.binary_search(&String::from("connect")).unwrap();
        netclient.connect(args.get(res + 1).unwrap().to_string());
    } else {
        netclient.connect("127.0.0.1".to_string());
    }

    // netclient.recieve();



    //TODO: Break this down into separate call////////////////////
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::PNG).unwrap();

    let window = video_subsystem.window("My new SDL Window", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().accelerated().present_vsync().build().unwrap();

    let texture_creator = canvas.texture_creator();

    let mut path = std::env::current_dir().unwrap().to_owned();
    
    path.push("fontaa.ttf");
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font(path,32).unwrap();
    font.set_style(sdl2::ttf::FontStyle::NORMAL);
    

    //let texture = texture_creator.load_texture("face.png")?;
    let catman = texture_creator.load_texture("catman.png")?;
    let players = texture_creator.load_texture("characters.png")?;
    let asset_pack = texture_creator.load_texture("First_Asset_pack.png")?;
    let grass = texture_creator.load_texture("Grass2.png")?;

    //////////////////////////////////////

    //let house_rect = Rect::new(36,0,48,96); //Where in the image do we want to source from?
    //let screen_center = Rect::new(0,0,200,200);

    let road_rect = Rect::new(216,156,36,36);


    let mut img_hash: HashMap<u8,Rect> = HashMap::new();
    create_player_images(&players, &mut img_hash);

    let mut player = Player
    {
        position: Point::new(0, 0), 
        sprite: Rect::new(0,0,16,32), 
        speed: 1000.0,
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

    let mut grid_map: HashMap<(i32,i32),Tile> = HashMap::new();
    
    create_hash_grid(&rows, &columns, &mut grid_map);

    let mut prevPlayerPos = player.position;

    let mut playerpositions: HashMap<u8, (i32, i32)> = HashMap::new();

    let mut random_keys: Vec<(i32,i32)> = vec![];

    let mut tile_paths: Vec<Vec<(i32,i32)>> = vec![];
    
    let mut random_tiles: Vec<Vec<Tile>> = vec![];

    let perlin = Perlin::default();

    let mut previousticks: f64 = unsafe { SDL_GetPerformanceCounter() as f64 };


    
    //Perlin LMAOOO
    for i in 0..10 {

        let random_key1: (i32,i32) = (rand::thread_rng().gen_range(0..16),rand::thread_rng().gen_range(0..9));
        let random_key2: (i32,i32) = (rand::thread_rng().gen_range(0..16),rand::thread_rng().gen_range(0..9));
        
        let point1 = [random_key1.0 as f64, random_key1.1 as f64];
        let raw_noise1 = perlin.get(point1);
        let noise1 = ((raw_noise1 + 1.0) / 2.0 * 16.0).round() as i32;

        let point2 = [random_key2.0 as f64, random_key2.1 as f64];
        let raw_noise2 = perlin.get(point2);
        let noise2 = ((raw_noise2 + 1.0) / 2.0 * 16.0).round() as i32;

        
        let random_key = (noise1,noise2);

        random_keys.push(random_key);
    }
    
    for key in random_keys.iter_mut() {

        allocate_nearby_tiles(&mut grid_map, key);
    }

    //Check difference between the two tiles
    //println!("X Diff: {}, Y Diff: {} ",random_keys[0].0 - random_keys[1].0, random_keys[0].1 - random_keys[1].1);


    // if let Some(a) = random_keys.get(1) {
    //     println!("{} : {}",a.0,a.1)
    // }
   
    'running: loop {

        
        let ticks:f64 = unsafe { SDL_GetPerformanceCounter() as f64 };

        let deltaticks = ticks - previousticks;

        previousticks = ticks;

        let _deltatime = deltaticks  / unsafe { SDL_GetPerformanceFrequency() as f64 };      

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
                
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => { player_input.keyboard_num = 1;}
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => { player_input.keyboard_num = 2;}
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => { player_input.keyboard_num = 3;}


                _ => {}
            }
        }

        

        let mut a = Vector2::new(0.0,0.0);
        check_player_input(&player_input, &mut a);
        a = a.normalize();
        
        player.position = player.position.offset((-player.speed * a.x * (_deltatime as f32)) as i32,(-player.speed * a.y * (_deltatime as f32)) as i32);

        mouse_position = (event_pump.mouse_state().x(),event_pump.mouse_state().y()).into();
        
        //Send local position to server
        if player.position != prevPlayerPos {
            // netclient.sendpos(Custom_Vector2 {x: player.position.x, y: player.position.y});
            //TODO call netclient.send and write netclient submit
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
        draw_tile_grid(&mut grid_map, &mut canvas, &grass);

        

        for playerclient in &playerpositions {

            player.sprite = img_hash[playerclient.0];

            render_players(Color::RED,&mut canvas,playerclient,&mut player);

            let player_text = "Player ".to_owned() + &playerclient.0.to_string();
            let text_surface = font.render(&player_text.to_string()).blended(Color::RGBA(255,0,0,255)).unwrap();
            let binding = canvas.texture_creator();
            let text_texture = binding.create_texture_from_surface(&text_surface).unwrap();

            render_text(&mut canvas, &text_texture, playerclient).unwrap();

        }



        for i in 1..random_keys.len() {
            let mut random_tile_x= 0;
            draw_paths(&mut tile_paths,&random_keys, i, &mut grid_map, &mut random_tile_x, &mut canvas,&asset_pack);
        }

        for i in 0..random_keys.len()  {
            if let Some(a) = grid_map.get_mut(&random_keys[i])
            {
                match &a.furniture {
                    Some(furniture) => canvas.copy(&asset_pack,furniture.sprite,furniture.rect).unwrap(),
                    _ => () 
                    }
            } 
        }        

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
                if piece.1.imageid == 1 {
                    match &piece.1.furniture {
                    Some(furniture) => canvas.copy(&asset_pack,furniture.sprite,furniture.rect).unwrap(),
                     _ => () 
                }
            } 
        }
        
        canvas.present();


        tile_paths.clear();

    }
    Ok(())

}

fn allocate_nearby_tiles(grid_map: &mut HashMap<(i32, i32), Tile>, key: &mut (i32, i32)) {
    let mut keys_to_modify: Vec<(i32,i32)> = vec![];

    loop {
        if check_and_update_key(&*grid_map, key, (0, 0))
            || check_and_update_key(&*grid_map, key, (1, 1))
            || check_and_update_key(&*grid_map, key, (0, 1))
            || check_and_update_key(&*grid_map, key, (1, 0))
        {
            continue;
        }
        break;

    }
    
    if let Some(a) = grid_map.get_mut(&key)
    {
        allocate_object(a, 1);
    
        if let Some(b) = &a.furniture {
            for i  in -b.object_width+1 ..  b.object_width {
                for j  in -b.object_height+1 ..  b.object_height {
                    keys_to_modify.push((key.0+i,key.1+j));
                } 
            } 
        }
    }
    
    for key in keys_to_modify {
        if let Some(a) = grid_map.get_mut(&key) {
            a.occupied = true;
        }
    }
}

fn draw_paths(
    path_collection: &mut Vec<Vec<(i32,i32)>>,
    random_keys: &Vec<(i32, i32)>, i: usize, 
    grid_map: &mut HashMap<(i32, i32), Tile>, 
    random_tile_x: &mut i32, 
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    road_texture: &Texture
) {

    let road_rect = Rect::new(216,156,36,36);

    let mut tile_path: Vec<(i32,i32)> = vec![];

    if &random_keys[i-1].0 - &random_keys[i].0 < 0 {
        for k in (0..=(&random_keys[i-1].0 - &random_keys[i].0).abs()).rev() {

            if let Some(a) = grid_map.get_mut(&(&random_keys[i as usize]))
            {
                let b = Rect::new(a.rect.x + ( a.rect.w * -k),a.rect.y,a.rect.width(),a.rect.height());
                *random_tile_x = &random_keys[i-1].0 - &random_keys[i].0;

                let tile_key = (b.x / 100,b.y/100);

                if let Some(path_tile) = grid_map.get_mut(&tile_key)
                {
                    path_tile.occupied = true;
                }

                tile_path.push((b.x,b.y));

                canvas.copy(&road_texture, road_rect, b).unwrap();

                //canvas.set_draw_color(Color::RED);
                //canvas.fill_rect(b).unwrap();
            }
        }
    }
    else
    {
        for k in 0..((&random_keys[i-1].0 - &random_keys[i].0)+1) {

            if let Some(a) = grid_map.get_mut(&(&random_keys[i as usize]))
            {
                let b = Rect::new(a.rect.x + ( a.rect.w * k),a.rect.y,a.rect.width(),a.rect.height());

                let tile_key = (b.x / 100,b.y/100);

                if let Some(path_tile) = grid_map.get_mut(&tile_key)
                {
                    path_tile.occupied = true;
                }
                
                *random_tile_x = k;

                tile_path.push((b.x,b.y));

                canvas.copy(&road_texture, road_rect, b).unwrap();
                // canvas.set_draw_color(Color::RED);
                // canvas.fill_rect(b).unwrap();
            }
        }
    }


    if &random_keys[i-1].1 - &random_keys[i].1 < 0 {
        for k in (0..=(&random_keys[i-1].1 - &random_keys[i].1).abs()).rev() {

            if let Some(a) = grid_map.get_mut(&(&random_keys[i as usize]))
            {
                let b = Rect::new(a.rect.x + ( a.rect.w * *random_tile_x),a.rect.y + ( a.rect.h * -k),a.rect.width(),a.rect.height());

                let tile_key = (b.x / 100,b.y/100);

                if let Some(path_tile) = grid_map.get_mut(&tile_key)
                {
                    path_tile.occupied = true;
                }

                tile_path.push((b.x,b.y));

                canvas.copy(&road_texture, road_rect, b).unwrap();
                // canvas.set_draw_color(Color::RED);
                // canvas.fill_rect(b).unwrap();
            }
        }
    }
    else
    {
        for k in 0..((&random_keys[i-1].1 - &random_keys[i].1)+1) {

            if let Some(a) = grid_map.get_mut(&(&random_keys[i as usize]))
            {
                let b = Rect::new(a.rect.x + ( a.rect.w * *random_tile_x),a.rect.y + ( a.rect.h * k),a.rect.width(),a.rect.height());    

                let tile_key = (b.x / 100,b.y/100);

                if let Some(path_tile) = grid_map.get_mut(&tile_key)
                {
                    path_tile.occupied = true;
                }         
                tile_path.push((b.x,b.y));
                canvas.copy(&road_texture, road_rect, b).unwrap();

                // canvas.set_draw_color(Color::RED);
                // canvas.fill_rect(b).unwrap();
            }
        }
    }


    path_collection.push(tile_path);


}

fn check_and_update_key(grid_map: &HashMap<(i32, i32), Tile>, key: &mut (i32, i32), offset: (i32, i32)) -> bool {
    if let Some(a) = grid_map.get(&(key.0 + offset.0, key.1 + offset.1)) {
        if a.occupied {
            *key = (
                rand::thread_rng().gen_range(0..15),
                rand::thread_rng().gen_range(0..8),
            );
            return true;
        }
    }
    false
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

fn draw_tile_grid(_tile_map: &mut HashMap<(i32,i32),Tile>,canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, road_texture: &Texture) {
    
    let road_rect = Rect::new(224,160,32,32);

    for tile in  _tile_map{
        if tile.1.highlight {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
            canvas.fill_rect(tile.1.rect).unwrap();
        }

        else if !tile.1.highlight {

            canvas.copy(&road_texture, road_rect, tile.1.rect).unwrap();

            //canvas.set_draw_color(Color::RGB(255, 255, 255));
            //canvas.draw_rect(tile.1.rect).unwrap();  
        }
    }
}

fn check_hash_tile(_tile_map: &mut HashMap<(i32,i32),Tile>,
    mouse_position: &Point, 
    player_input: &player_module::PlayerInput) 
    {

    let key = &(mouse_position.x / 100,mouse_position.y/ 100);
    let mut keys_to_modify = Vec::new();
    let mut value_to_modify_to = true;


    for tile in _tile_map.into_iter() {

        if tile.1.position.x == key.0 && tile.1.position.y == key.1{
            tile.1.highlight = true;

            if player_input.m1_is_down && !tile.1.occupied{

                allocate_object(tile.1, player_input.keyboard_num);
                value_to_modify_to = true;

                if let Some(a) = &tile.1.furniture {
                    
                    //TODO: for tile in (key)(height,width) -> return tiles there
                    for i  in -a.object_width+1 ..  a.object_width {
                        for j  in -a.object_height+1 ..  a.object_height {
                            keys_to_modify.push((key.0+i,key.1+j));
                        } 
                    }    
                }
            }
            if player_input.m2_is_down && tile.1.occupied{

                if let Some(a) = &tile.1.furniture {
                    
                    for i  in -a.object_width+1 ..  a.object_width {
                        keys_to_modify.push((key.0+i,key.1));
                    }   
                }

                tile.1.occupied = false;
                tile.1.furniture = None;
                value_to_modify_to = false;
            }   
        }
        else {
        tile.1.highlight = false;    
        }
    }

    let mut indices_to_remove = Vec::new();
    for (index, tile) in keys_to_modify.iter().enumerate() {
        if let Some(a) = _tile_map.get_mut(tile) {
            a.occupied = value_to_modify_to;
            indices_to_remove.push(index);
        }
    }


    for index in indices_to_remove.into_iter().rev() {
        keys_to_modify.remove(index);
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



