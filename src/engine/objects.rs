
pub mod object_module{
    use sdl2::rect::Rect;
    use crate::engine::Tile;

pub struct Objects
{
    pub rect: Rect,
    pub sprite: Rect,
    pub object_width: i32,
    pub object_height: i32
}
    
    pub fn allocate_object(tile: &mut Tile, num: u32) {
        
        if num == 0 {
            let sprite_rect = Rect::new(0,0, 384, 128);

            let furn_rect = Rect::new(
            tile.rect.x + (tile.rect.width() / 2) as i32,
            tile.rect.y + (tile.rect.height() / 2) as i32,
            tile.rect.width() * 3,
            tile.rect.height());
    
            let new_piece = Objects {
                rect: furn_rect,
                sprite: sprite_rect,
                object_width: 3,
                object_height: 1
            };

            tile.imageid = 0;
            tile.occupied = true;
            tile.furniture = Some(new_piece);
        }
        if num == 1 {
            let sprite_rect = Rect::new(36,0,48,96);

            let furn_rect = Rect::new(
            tile.rect.x as i32,
            tile.rect.y as i32,
            tile.rect.width() * 1,
            tile.rect.height() * 1);
    
            let new_piece = Objects {
                rect: furn_rect,
                sprite: sprite_rect,
                object_width: 1,
                object_height: 1
            };
            tile.imageid = 1;
            tile.occupied = true;
            tile.furniture = Some(new_piece);

       }
       if num == 2 {
        let road_rect = Rect::new(216,156,36,36);

        let furn_rect = Rect::new(
        tile.rect.x as i32,
        tile.rect.y as i32,
        tile.rect.width(),
        tile.rect.height());

        let new_piece = Objects {
            rect: furn_rect,
            sprite: road_rect,
            object_width: 1,
            object_height: 1
        };
        tile.imageid = 2;
        tile.occupied = true;
        tile.furniture = Some(new_piece);

   }
       

        
                            
    }
}