
pub mod object_module{
    use sdl2::{pixels::Color, rect::Rect};
    use crate::engine::Tile;

pub struct Objects
{
    pub rect: Rect,
    pub sprite: Rect,
}


    pub fn place_furniture(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>, tile: &mut Tile, sprite_rect: &mut Rect, tile_rect: &mut Rect) {
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.draw_rect(tile.rect).unwrap();
        
    *sprite_rect = Rect::new(0,0, 384, 128);

        tile_rect.set_x(tile.rect.x + (tile.rect.width() / 2) as i32);
        tile_rect.set_y(tile.rect.y + (tile.rect.height() / 2) as i32);
        tile_rect.set_width(tile.rect.width() * 3);
        tile_rect.set_height(tile.rect.height());

                            
                            
        let new_piece = Objects {
            rect: *tile_rect,
            sprite: *sprite_rect
        };

        tile.occupied = true;
        tile.furniture = Some(new_piece);

        println!("x{} : y{}",tile.position.x,tile.position.y)
                            
    }
}