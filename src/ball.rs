use gdnative::{
    api::{KinematicBody2D, Node},
    nativescript::init::property::{FloatHint, RangeHint, Usage},
    prelude::{godot_print, methods, user_data::MutexData, GodotString, Vector2},
};

use rand::{
    distributions::{Distribution, Uniform},
    Rng,
};

use crate::utils::clamp;

const BASE_SPEED: f32 = 1.0;
const MIN_VELOCITY_X: f32 = 2.5;
const MAX_VELOCITY_X: f32 = 5.0;
const MIN_VELOCITY_Y: f32 = 2.5;
const MAX_VELOCITY_Y: f32 = 5.0;

pub struct Ball {
    speed: f32,
    movement_vec: Vector2,
    rng: rand::prelude::ThreadRng,
    coin: Uniform<i32>,
}

unsafe impl Send for Ball {}

impl gdnative::prelude::NativeClass for Ball {
    type Base = KinematicBody2D;
    type UserData = MutexData<Ball>;

    fn class_name() -> &'static str {
        "Ball"
    }

    fn init(owner: &Self::Base) -> Self {
        Self::new(owner)
    }

    fn register_properties(_builder: &gdnative::nativescript::init::ClassBuilder<Self>) {
        _builder
            .add_property("Speed")
            .with_default(BASE_SPEED) // sets the default value for the ball object
            .with_hint(FloatHint::from(RangeHint::new(0.0, 500.0)))
            .with_getter(|this: &Ball, _owner: &KinematicBody2D| this.speed) //creates a getter for the ball object
            .with_setter(|this: &mut Ball, _owner: &KinematicBody2D, v| this.speed = v) //creates a setter for the ball object
            .with_usage(Usage::DEFAULT)
            .done();
    }
}

#[methods]
impl Ball {
    fn new(_owner: &KinematicBody2D) -> Self {
        Ball {
            speed: BASE_SPEED,
            movement_vec: Vector2::zero(),
            rng: rand::thread_rng(),
            coin: Uniform::from(0..2),
        }
    }

    #[export]
    unsafe fn _ready(&mut self, _owner: &KinematicBody2D) {
        match self.coin.sample(&mut self.rng) {
            0 => self.movement_vec.x = -MIN_VELOCITY_X,
            1 => self.movement_vec.x = MIN_VELOCITY_X,
            _ => godot_print!("[ERROR] Did we flip the coin onto its side?"),
        }

        godot_print!("Ball created!");
    }

    #[export]
    unsafe fn _physics_process(&mut self, owner: &KinematicBody2D, _delta: f64) {
        self.movement_vec *= self.speed;
        match owner.move_and_collide(self.movement_vec, true, true, false) {
            Some(collision_data) => {
                let collider = collision_data
                    .assume_unique()
                    .collider()
                    .unwrap()
                    .assume_unique()
                    .cast::<Node>()
                    .unwrap();
                if collider.is_in_group(GodotString::from_str("Paddles")) {
                    match self.coin.sample(&mut self.rng) {
                        0 => {
                            let random_x = self.rng.gen_range(-3.0..3.0);
                            self.movement_vec.x = -self.movement_vec.x;
                            if self.movement_vec.x < 0.0 {
                                self.movement_vec.x = clamp(
                                    self.movement_vec.x + random_x,
                                    -MAX_VELOCITY_X,
                                    -MIN_VELOCITY_X,
                                );
                            } else {
                                self.movement_vec.x = clamp(
                                    self.movement_vec.x + random_x,
                                    MIN_VELOCITY_X,
                                    MAX_VELOCITY_X,
                                );
                            }
                        }
                        1 => {
                            let random_y = self.rng.gen_range(-3.0..3.0);
                            self.movement_vec.y = -self.movement_vec.y;
                            if self.movement_vec.y < 0.0 {
                                self.movement_vec.y = clamp(
                                    self.movement_vec.y + random_y,
                                    -MAX_VELOCITY_Y,
                                    -MIN_VELOCITY_Y,
                                );
                            } else {
                                self.movement_vec.y = clamp(
                                    self.movement_vec.y + random_y,
                                    MIN_VELOCITY_Y,
                                    MAX_VELOCITY_Y,
                                );
                            }
                        }
                        _ => godot_print!("[ERROR] How did we mess up a coin flip?"),
                    }
                }
                if collider.is_in_group(GodotString::from_str("Walls")) {
                    self.movement_vec.y = -self.movement_vec.y;
                }
            }
            _ => (),
        }
    }
}
