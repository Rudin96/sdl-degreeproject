use sdl2::{render::{WindowCanvas, Texture}, pixels::Color, rect::{Point, Rect}, ttf::Font};

use crate::engine::player_module::Player;

pub fn render_players(color: Color,canvas: &mut WindowCanvas,playerclient: (&u8,&(i32, i32)),player: &Player)
{

    canvas.set_draw_color(color);

    let (width, height) = canvas.output_size().unwrap();

    let playpos = Point::new(playerclient.1.0,playerclient.1.1);

    let screen_position = playpos + Point::new(width as i32/2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());

    canvas.copy(&player.player_texture, player.sprite, screen_rect).unwrap();
}


//Break down into draw text and draw player.
// pub fn render_player(canvas: &mut WindowCanvas, color: Color,player: &Player,font: &Font) -> Result<(), String>{ 

//     canvas.set_draw_color(color);

//     let (width, height) = canvas.output_size().unwrap();

//     let screen_position = player.position + Point::new(width as i32/2, height as i32 / 2);
//     let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());

//     canvas.copy(&player.player_texture, player.sprite, screen_rect).unwrap();

    
//     let player_text = "Player ".to_owned() + &player.player_id.to_string();
//     let text_surface = font.render(&player_text.to_string()).blended(Color::RGBA(255,0,0,255)).unwrap();
//     let binding = canvas.texture_creator();
//     let text_texture = binding.create_texture_from_surface(&text_surface).unwrap();

//     //render_text(canvas, color, &text_texture, player).unwrap();    

//     Ok(())
// }

// pub fn render_text(canvas: &mut WindowCanvas, color: Color, text_texture: &Texture,player: &Player) -> Result<(), String>{ 

//     canvas.set_draw_color(color);

//     let (width, height) = canvas.output_size()?;

//     let screen_position = player.position + Point::new(width as i32/2, (height as i32 / 2) - 32) ;
//     let screen_rect = Rect::from_center(screen_position, text_texture.query().width, text_texture.query().height);

    

//     canvas.copy(text_texture, None, screen_rect).unwrap();

//     Ok(())
// }

pub fn render_text(canvas: &mut WindowCanvas, text_texture: &Texture,playerclient: (&u8,&(i32, i32))) -> Result<(), String>{ 

    canvas.set_draw_color(Color::RGB(255, 0, 0));

    let (width, height) = canvas.output_size()?;

    let playpos = Point::new(playerclient.1.0,playerclient.1.1);

    let screen_position = playpos + Point::new(width as i32/2, (height as i32 / 2) - 32) ;
    let screen_rect = Rect::from_center(screen_position, text_texture.query().width, text_texture.query().height);

    canvas.copy(text_texture, None, screen_rect).unwrap();

    Ok(())
}


