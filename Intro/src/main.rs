mod app_window;
mod game;

use winit::event_loop::{EventLoop, ControlFlow};
use app_window::window_creation::App;
use game::tile::Tile;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App{
        window: None,
        renderer: None,
    };

    event_loop.run_app(&mut app).unwrap();

    
}


