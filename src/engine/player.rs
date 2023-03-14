


pub mod player_module{


    use sdl2::{rect::{Rect, Point}, render::Texture, pixels::Color};

    //#[derive(Debug)]
    pub struct Player<'a>
    {
        pub position: Point,
        pub sprite: Rect,
        pub speed: i32,
        pub player_texture: Texture<'a>
    }

    pub struct PlayerText<'a>
    {
        pub text_texture: Texture<'a>,
        pub text: String,
        pub text_color: Color
    }

    // impl Default for Player<'_>{
    //     fn default() -> Self {
    //         Self { position: Point::new(0, 0), sprite: Rect::new(0,0,32,32), speed: 5,player_texture: None }
    //     }
    // }

    pub struct PlayerInput{

        pub left_is_held_down: bool,
        pub right_is_held_down: bool,
        pub up_is_held_down: bool,
        pub down_is_held_down: bool,
        
        pub m1_is_down: bool,
        pub m2_is_down: bool,
    }

    impl Default for PlayerInput {
        fn default() -> Self {
            Self { left_is_held_down: false, right_is_held_down: false, up_is_held_down: false, down_is_held_down: false, m1_is_down: false, m2_is_down: false }
        }
    }
}