


pub mod player_module{

    use sdl2::rect::{Rect, Point};

    #[derive(Debug)]
    pub struct Player
    {
        pub position: Point,
        pub sprite: Rect,
        pub speed: i32
    }

    impl Default for Player{
        fn default() -> Self {
            Self { position: Point::new(0, 0), sprite: Rect::new(0,0,32,32), speed: 5 }
        }
    }

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