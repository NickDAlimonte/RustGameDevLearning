
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::window::Window;
use crate::game::game_world::GameWorld;

pub struct Renderer {
    pixels: Pixels<'static>,
    width: u32,
    height: u32,
}
impl Renderer{

    pub fn new(window: Arc<Window>, width: u32, height: u32) -> Self{
        let surface_texture = SurfaceTexture::new(width, height, window);

        let pixels = Pixels::new(width, height, surface_texture).expect("Failed to create pixels");
        Self { pixels, width, height }
        }

    pub fn render(&mut self){
        {
            let frame = self.pixels.frame_mut();

            for pixel in frame.chunks_exact_mut(4) {
                pixel[0] = 255;
                pixel[1] = 255;
                pixel[2] = 255;
                pixel[3] = 255;
            }
        }

        
        self.pixels.render().expect("Failed to render pixels");
    }

    pub fn resize(&mut self, width: u32, height: u32){
        self.width = width;
        self.height = height;

        self.pixels
            .resize_surface(width, height)
            .expect("Failed to resize pixels surface");

        self.pixels.resize_buffer(width, height).expect("Failed to resize pixels buffer");
    }
}
pub struct Square{
    width: u32,
    height: u32
}