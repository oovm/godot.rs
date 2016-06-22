use godot::{
    bind::{godot_api, GodotClass},
    engine::{ISprite2D, Sprite2D},
    log::godot_print,
    obj::{Base, WithBaseField},
};

#[derive(Debug, GodotClass)]
#[class(base = Sprite2D)]
pub struct RustPlayer {
    #[base]
    base: Base<Sprite2D>,
    speed: f64,
    angular_speed: f64,
}

#[godot_api]
impl ISprite2D for RustPlayer {
    fn init(base: Base<Sprite2D>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        Self { speed: 400.0, angular_speed: std::f64::consts::PI, base }
    }
    fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be:
        // rotation += angular_speed * delta

        let radians = (self.angular_speed * delta) as f32;
        self.base_mut().rotate(radians);
        // The 'rotate' method requires a f32,
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    }
}

#[derive(Clone, Debug, GodotClass)]
struct RustMonster {
    name: String,
    hit_points: i32,
}
