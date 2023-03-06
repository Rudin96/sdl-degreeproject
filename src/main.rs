use std::collections::HashSet;
use std::env;
use std::path::Path;
use std::time::Duration;
use sdl2::pixels::Color;
use sdl2::event::{Event, self};
use sdl2::keyboard::{Keycode, Scancode};
use sdl2::rect::{Rect, Point};
use sdl2::render::{WindowCanvas, Texture, TextureQuery};
use sdl2::sys::{SDL_CommonEvent, SDL_Keycode, Window, ttf};
use sdl2::image::{self,LoadTexture,InitFlag};

macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

#[derive(Debug)]
struct Player
{
    position: Point,
    sprite: Rect,
    speed: i32
}


fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
    let wr = rect_width as f32 / cons_width as f32;
    let hr = rect_height as f32 / cons_height as f32;

    let (w, h) = if wr > 1f32 || hr > 1f32 {
        if wr > hr {
            println!("Scaling down! The text will look worse!");
            let h = (rect_height as f32 / wr) as i32;
            (cons_width as i32, h)
        } else {
            println!("Scaling down! The text will look worse!");
            let w = (rect_width as f32 / hr) as i32;
            (w, cons_height as i32)
        }
    } else {
        (rect_width as i32, rect_height as i32)
    };

    let cx = (600 as i32 - w) / 2;
    let cy = (800 as i32 - h) / 2;
    rect!(cx, cy, w, h)
}


fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture,player: &Player) -> Result<(), String>{ 

    canvas.set_draw_color(color);
    canvas.clear();
    
    let (width, height) = canvas.output_size()?;

    let screen_position = player.position + Point::new(width as i32/2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());

    canvas.copy(texture, player.sprite, screen_rect);

   

    canvas.present();

    Ok(())

}


// fn find_sdl_gl_driver() -> Option<u32> {
//     for (index, item) in sdl2::render::drivers().enumerate() {
//         if item.name == "opengl" {
//             return Some(index as u32);
//         }
//     }
//     None
// }

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::PNG)?;

    let window = video_subsystem.window("My new SDL Window", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();


    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("face.png")?;

    let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_owned();
    path.push("fontaa.ttf");

    let ttf_context = sdl2::ttf::init().unwrap();
    let mut font = ttf_context.load_font(path,128)?;

    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let text_surface = font.render("hello RSOAT").blended(Color::RGBA(255,0,0,255)).unwrap();
    let text_texture = texture_creator.create_texture_from_surface(&text_surface).unwrap();



    let mut player = Player { 
        position: Point::new(0, 0), 
        sprite: Rect::new(0,0,32,32), 
        speed: 5 
    };

    

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();


    let TextureQuery {width,height,..} = text_texture.query();

    let target = get_centered_rect(
        width,
        height,
        600 - 64,
        800 - 64,
    );
    
    

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut left_is_held_down = false;
    let mut right_is_held_down = false;
    let mut up_is_held_down = false;
    let mut down_is_held_down = false;




    let mut i = 0;

    'running: loop {

        


        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255-i));
        canvas.clear();

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
        } 
        else if right_is_held_down {
            player.position = player.position.offset(player.speed, 0);
        } 
        else if up_is_held_down {
            player.position = player.position.offset(0, -player.speed);
        }
        else if down_is_held_down {
            player.position = player.position.offset(0, player.speed);
        }

        render(&mut canvas, Color::RGB(i, 64, 255 - i), &texture,&player);

        canvas.present();

        canvas.copy(&text_texture, None, Some(target)).unwrap();
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        
    }

    Ok(())
    // ...
}