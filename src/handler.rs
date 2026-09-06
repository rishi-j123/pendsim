use std::f32::consts::PI;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::pendulum::{self, Pendulum};
use nalgebra::vector;
use speedy2d::color::Color;
use speedy2d::{dimen::Vector2, window::WindowHandler};

pub struct Handler {
    timer: Instant,
    start: Instant,
    win_size: Vector2<f32>,
    mouse_pos: (f32, f32),

    pendulum: Pendulum,
}

impl Handler {
    pub fn new() -> Handler {
        let pendulum = Pendulum::new(
            vector![0.0, 0.0],
            vector![0.0, 0.0],
            vector![0.0, 0.0],
            vector![0.0, -1.0],
            0.0,
            0.0,
            10.0,
            0.127,
        );

        Handler {
            timer: Instant::now(),
            start: Instant::now(),
            win_size: Vector2 { x: 0.0, y: 0.0 },
            mouse_pos: (0.0, 0.0),

            pendulum,
        }
    }
}

impl WindowHandler for Handler {
    fn on_draw(
        &mut self,
        h: &mut speedy2d::window::WindowHelper<()>,
        g: &mut speedy2d::Graphics2D,
    ) {
        // setup
        g.clear_screen(Color::BLACK);
        let dt = self.timer.elapsed().as_secs_f32();
        self.timer = Instant::now();

        // calc
        self.pendulum.update(dt);
        let (bob_loc, bob_rad) = self.pendulum.get_bob_drawing();
        let origin = self.pendulum.get_origin_loc();

        println!("{:.2?}", self.pendulum);

        // draw
        g.draw_circle(
            to_window_coordinates(self.win_size, origin),
            5.0,
            Color::RED,
        );
        g.draw_circle(
            to_window_coordinates(self.win_size, bob_loc),
            bob_rad,
            Color::WHITE,
        );

        h.request_redraw();
    }

    fn on_resize(
        &mut self,
        _helper: &mut speedy2d::window::WindowHelper<()>,
        size_pixels: speedy2d::dimen::UVec2,
    ) {
        self.win_size = size_pixels.into_f32();
    }

    fn on_mouse_move(
        &mut self,
        _helper: &mut speedy2d::window::WindowHelper<()>,
        position: speedy2d::dimen::Vec2,
    ) {
        let x = position.x - (self.win_size.x / 2.0);
        let y = (self.win_size.y / 2.0) - position.y;

        self.mouse_pos = (x, y);
    }
}

fn to_window_coordinates(win_size: Vector2<f32>, coordinate: (f32, f32)) -> (f32, f32) {
    (
        coordinate.0 + (win_size.x / 2.0),
        -coordinate.1 + (win_size.y / 2.0),
    )
}
