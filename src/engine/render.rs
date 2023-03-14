use sdl2::{render::{WindowCanvas, Texture}, pixels::Color, rect::{Point, Rect}};

use crate::engine::player_module::Player;





pub fn render_player(canvas: &mut WindowCanvas, color: Color,player: &Player) -> Result<(), String>{ 

    canvas.set_draw_color(color);

    let (width, height) = canvas.output_size().unwrap();

    let screen_position = player.position + Point::new(width as i32/2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());

    canvas.copy(&player.player_texture, player.sprite, screen_rect).unwrap();

    match player.text_texture {
        Some(ref texture) => {
            // Pass the texture reference to the function
            render_text(canvas, color, &texture, player).unwrap();
        }
        None => {
            // Handle the case where text_texture is None
        }
    }

    

    Ok(())
}

pub fn render_text(canvas: &mut WindowCanvas, color: Color, text_texture: &Texture,player: &Player) -> Result<(), String>{ 

    canvas.set_draw_color(color);

    let (width, height) = canvas.output_size()?;

    let screen_position = player.position + Point::new(width as i32/2, (height as i32 / 2) - 32) ;
    let screen_rect = Rect::from_center(screen_position, text_texture.query().width, text_texture.query().height);

    

    canvas.copy(text_texture, None, screen_rect).unwrap();

    Ok(())
}


