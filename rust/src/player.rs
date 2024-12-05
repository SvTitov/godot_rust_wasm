use godot::obj::NewAlloc;
use godot::prelude::*;
use godot::classes::{ISprite2D, Sprite2D};
use godot::register::{Export};

#[derive(GodotClass)]
#[class(base=Sprite2D)]
struct Player {
    speed: f64,

    #[export]
    angular_speed: f64,

    base: Base<Sprite2D>
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base < Self::Base >) -> Self {
        godot_print!("Hello, world!");

        Self {
            speed: 100.0,
            angular_speed: std::f64::consts::PI,
            base
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
    }
}

#[godot_api]
impl Player {
    #[signal]
    fn speed_inc();
}