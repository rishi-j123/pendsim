use nalgebra::{Rotation2, Vector2, vector};
use std::f32::consts::PI;

use crate::constants::{DRAG_COEFF, RHO};

#[derive(Clone, Debug)]
pub struct Pendulum {
    // meters
    pub origin: Vector2<f32>,
    // meters / s
    origin_velocity: Vector2<f32>,
    // meters
    bob: Vector2<f32>,
    // radians / s
    bob_omega: f32,
    // radians / s^2
    bob_alpha: f32,
    // kilograms
    bob_weight: f32,
    // meters
    bob_radius: f32,
    // meters
    length: f32,
}

impl Pendulum {
    pub fn new(
        origin: Vector2<f32>,
        origin_velocity: Vector2<f32>,
        bob: Vector2<f32>,
        bob_omega: f32,
        bob_alpha: f32,
        bob_weight: f32,
        bob_radius: f32,
    ) -> Pendulum {
        let length = bob.magnitude();
        Self {
            origin,
            origin_velocity,
            bob,
            bob_omega,
            bob_alpha,
            bob_weight,
            bob_radius,
            length,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let gravity = vector![0.0, -9.8];
        self.apply_force(&gravity);
        self.apply_air_res();
        self.bob_omega += self.bob_alpha * 1.0;
        self.apply_omega(&1.0);
    }

    /// updates acceleration
    fn apply_force(&mut self, force: &Vector2<f32>) {
        let tangent_vec = Rotation2::new(self.get_bob_angle()) * Vector2::new(0.0, 1.0);
        let tangential_force = force.dot(&tangent_vec) * tangent_vec.clone();
        let tangential_accel = tangential_force.map(|force| force / self.bob_weight);
        let direction = tangential_accel.dot(&tangent_vec).signum();
        self.bob_alpha = tangential_accel.magnitude() * direction / self.length;
    }

    fn apply_air_res(&mut self) {
        let velocity = self.bob_omega * self.bob_radius;
        let area = PI * self.bob_radius * self.bob_radius;

        let resistance_force = 0.5 * RHO * velocity * velocity * DRAG_COEFF * area;
        let resistance_accel = resistance_force / self.bob_weight;
        let direction = self.bob_omega.signum() * -1.0;
        self.bob_alpha += resistance_accel * direction / self.length;
    }

    fn apply_omega(&mut self, dt: &f32) {
        let delta_theta = self.bob_omega * dt;
        self.bob = Rotation2::new(delta_theta) * self.bob
    }

    fn get_bob_angle(&self) -> f32 {
        self.bob.y.atan2(self.bob.x)
    }

    /// Position, Radius
    pub fn get_bob_drawing(&self) -> ((f32, f32), f32) {
        let location = self.origin + self.bob;
        ((location.x, location.y), self.bob_radius)
    }

    pub fn get_origin_loc(&self) -> (f32, f32) {
        let location = self.origin;
        (location.x, location.y)
    }

    pub fn set_origin(&mut self, x: f32, y: f32) {
        self.origin = vector![x, y];
    }
}
