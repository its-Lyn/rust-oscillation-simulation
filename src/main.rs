use macroquad::color;
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};
use macroquad::math::Vec2;
use macroquad::shapes::{draw_line, draw_rectangle};
use macroquad::text::draw_text;
use macroquad::window::{clear_background, Conf, next_frame, screen_height, screen_width};

pub struct Body {
    pub position: Vec2,
    pub rest_position: Vec2,
    pub size: Vec2,

    pub mass: f32,
    pub k: f32,

    velocity: Vec2
}

impl Body {
    pub fn new(position: Vec2, size: Vec2, mass: f32, k: f32) -> Self {
        Body {
            position,
            rest_position: Vec2 {
                x: (screen_width() / 2.0) - 50.0,
                y: (screen_height() / 2.0) - 50.0
            },
            size,

            mass,
            k,

            velocity: Default::default()
        }
    }

    fn get_magnitude(&self, vector: Vec2) -> f32 {
        (vector.x.powi(2) + vector.y.powi(2)).sqrt()
    }

    fn calc_force(&self, displacement: f32) -> f32 {
        -self.k * displacement
    }

    pub fn update(&mut self) {
        if self.position == self.rest_position {
            return
        };

        let displacement_vec: Vec2 = self.position - self.rest_position;
        let force: f32 = self.calc_force(self.get_magnitude(displacement_vec));

        self.velocity += displacement_vec.normalize() * 0.016 * force / self.mass;
        self.position += 0.016 * self.velocity;
    }

    pub fn draw(&self) {
        draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, color::BLUE);
    }
}

fn create_window_conf() -> Conf {
    Conf {
        window_title: "Oscillation Simulation".to_string(),

        window_height: 600,
        window_width: 800,

        fullscreen: false,
        window_resizable: false,

        ..Default::default()
    }
}

#[macroquad::main(create_window_conf)]
async fn main() {
    let mut rect_body: Body = Body::new(
        Vec2 {
            x: (screen_width() / 2.0) - 50.0,
            y: screen_height() / 2.0 + 20.0
        },
        Vec2 {
            x: 100.0,
            y: 50.0
        },
        40.0,
        150.0
    );

    loop {
        rect_body.update();
        if rect_body.mass <= 6.0 {
            rect_body.mass = 6.0;
        }

        if is_key_pressed(KeyCode::Up) {
            rect_body.mass += 5.0;
        } else if is_key_pressed(KeyCode::Down) {
            rect_body.mass -= 5.0;
        }

        clear_background(color::WHITE);
        draw_text(format!("Echilibrium: {}", (screen_height() / 2.0) - 50.0).as_str(), 5.0, 15.0, 20.0, color::BLACK);
        draw_text(format!("Masa: {}", rect_body.mass).as_str(), 5.0, 27.0, 20.0, color::BLACK);
        draw_text(format!("K: {}", rect_body.k).as_str(), 5.0, 39.0, 20.0, color::BLACK);

        draw_line(
            screen_width() / 2.0,
            -5.0,
            screen_width() / 2.0,
            rect_body.position.y,
            5.0,
            color::BLACK
        );
        rect_body.draw();

        next_frame().await
    }
}