use sdl2::{render::{WindowCanvas, Texture}, pixels::Color, rect::{Point, Rect}};

use crate::Player;



pub fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture,player: &Player) -> Result<(), String>{ 

    canvas.set_draw_color(color);
    //canvas.clear();
    
    let (width, height) = canvas.output_size()?;

    let screen_position = player.position + Point::new(width as i32/2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());

    canvas.copy(texture, player.sprite, screen_rect).unwrap();

    Ok(())
}


// pub fn load_font(canvas: &mut &WindowCanvas,texture: &Texture) -> Texture
// {
//     let mut path = Path::new(env!("CARGO_MANIFEST_DIR")).to_owned();
//     path.push("fontaa.ttf");

//     let ttf_context = sdl2::ttf::init().unwrap();
//     let mut font = ttf_context.load_font(path,32).unwrap();

//     font.set_style(sdl2::ttf::FontStyle::BOLD);

//     let texture_creator = canvas.texture_creator();

//     let text_surface = font.render("AAAAA").blended(Color::RGBA(255,0,0,255)).unwrap();
//     let text_texture = &texture_creator.create_texture_from_surface(&text_surface).unwrap();

//     text_texture

//     // let (canvas_width, canvas_height) = canvas.output_size().unwrap();

//     // let screen_position = player.position + Point::new(canvas_width as i32/2, (canvas_height as i32 / 2) - 32) ;
//     // let screen_rect = Rect::from_center(screen_position, text_texture.query().width, text_texture.query().height);

// }
