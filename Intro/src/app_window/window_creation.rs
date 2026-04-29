use crate::app_window::rendering::Renderer;
use winit::{
    application::ApplicationHandler,
    event::{ WindowEvent },
    event_loop::{ActiveEventLoop},
    window::{Window, WindowAttributes,Fullscreen},
    dpi::{PhysicalSize, PhysicalPosition},
};
use std::sync::Arc;
use crate::game::game_world::GameWorld;

pub struct App {
    pub window: Option<Arc<Window>>,
    pub renderer: Option<Renderer>,
    pub world: GameWorld,
}
impl ApplicationHandler for App{

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {

        }else {
            let window = Arc::new(event_loop
                .create_window(WindowAttributes::default().with_visible(false))
                .unwrap()
            );

            let (width, height) = if let Some(monitor) = event_loop.primary_monitor() {
                let size = monitor.size();
                (size.width, size.height)
            } else {
                (640, 480)
            };
            let fullscreen_mode = false;

            if !fullscreen_mode {
            window.request_inner_size(PhysicalSize::new(width/2, height/2));
            }else{
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
            }

            let renderer = Renderer::new(Arc::clone(&window), width, height);

            self.window = Some(window);
            self.renderer = Some(renderer);

            if let Some(window) = &self.window {
                window.set_visible(true);
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: winit::window::WindowId, event: WindowEvent){

        let window = self.window.as_ref().unwrap();

        match event {
            WindowEvent::CloseRequested => {
                println!("Closing the window");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.render(&self.world);
                }

            }

            WindowEvent::Resized(new_size) => {
                if let Some(renderer) = &mut self.renderer {

                    const TILE_SIZE: u32 = 32;

                    let snapped_width = (new_size.width / TILE_SIZE) * TILE_SIZE;
                    let snapped_height = (new_size.height / TILE_SIZE) * TILE_SIZE;

                    renderer.resize(snapped_width, snapped_height);



                }
            }
            _ =>{}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop){
        if let Some(window) = &self.window {
        }
    }


}