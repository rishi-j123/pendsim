mod handler;
mod pendulum;
mod constants;

use crate::handler::Handler;
use speedy2d::Window;

fn main() {
    let window = Window::<()>::new_centered("yogurt", (1000, 1000)).unwrap();
    let handler = Handler::new();

    window.run_loop(handler);
}
