mod app_window;
mod game;

use winit::event_loop::{EventLoop, ControlFlow};
use app_window::window_creation::App;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);

    let new_world = game::game_world::GameWorld::new();

    let mut app = App{
        window: None,
        renderer: None,
        world: new_world,
    };

    event_loop.run_app(&mut app).unwrap();


    
}


