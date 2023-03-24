


pub mod player_module{


    use sdl2::{rect::{Rect, Point}, render::Texture};

    //#[derive(Debug)]
    pub struct Player<'a>
    {
        pub position: Point,
        pub sprite: Rect,
        pub speed: f32,
        pub player_texture: Texture<'a>,
        pub player_id: i32,

        pub text_texture: Option<Texture<'a>>
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
        
        pub tab: bool,

        pub keyboard_num: u32
    }

    impl Default for PlayerInput {
        fn default() -> Self {
            Self { 
                left_is_held_down: false, 
                right_is_held_down: false, 
                up_is_held_down: false, 
                down_is_held_down: false, 
                m1_is_down: false, 
                m2_is_down: false, 
                tab: true, 
                keyboard_num: 0 }
        }
    }
}