
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

    pub fn render(&mut self, world: &GameWorld){
        {
            let frame = self.pixels.frame_mut();

            for pixel in frame.chunks_exact_mut(4) {
                pixel[0] = 255;
                pixel[1] = 255;
                pixel[2] = 255;
                pixel[3] = 255;
            }
        }

        Self::render_world(world, self.width, self.height, self.pixels.frame_mut(), );

        Self::draw_cube(self.pixels.frame_mut(), self.width, self.height);
        //Self::draw_heart(self.pixels.frame_mut(), self.width, self.height);
        //Self::draw_circle(self.pixels.frame_mut(), self.width, self.height);

        
        self.pixels.render().expect("Failed to render pixels");
    }

    pub fn resize(&mut self, width: u32, height: u32){
        self.width = width;
        self.height = height;

        if width == 0 || height == 0 {return;}

        self.pixels
            .resize_surface(width, height)
            .expect("Failed to resize pixels surface");

        self.pixels.resize_buffer(width, height).expect("Failed to resize pixels buffer");
    }

    fn render_world(world: &GameWorld, frame_width: u32, frame_height: u32, frame: &mut [u8]){
        const TILE_SIZE: u32 = 32;
        let tiles_per_row =frame_width/  TILE_SIZE;

        for (index, tiles) in world.tiles.iter().enumerate() {
            Self::draw_tile(frame, frame_width,frame_height, TILE_SIZE, TILE_SIZE, tiles.color, (TILE_SIZE*(index as u32 % tiles_per_row), TILE_SIZE*(index as u32 / tiles_per_row)));
        }
    }

    fn draw_tile(frame: &mut [u8],frame_width: u32,frame_height: u32,  width: u32, height: u32, color: [u8; 4], position: (u32, u32)){
        let (start_x, start_y) = position;

        for y in 0..height{
            for x in 0..width{

                let px = start_x + x;
                let py = start_y + y;

                if px >= frame_width || py >= frame_height {
                    continue;
                }

                let pixel_index = ((py * frame_width + px) * 4) as usize;

                frame[pixel_index..pixel_index+4].copy_from_slice(&color);
            }

        }

    }

    fn draw_heart(frame: &mut [u8], frame_width: u32, frame_height: u32){
        let height = 100;
        let width = 100;

        let center_x = frame_width/2;
        let center_y = frame_height/2;

        let start_x = center_x - width/2;
        let start_y = center_y - height/2;

        let color: [u8;4] =  [255,0,0,255];


        for y in 0..height{
            for x in 0..width{
                let nx = (x as f32 / width as f32) * 2.0 - 1.0;
                let ny = (y as f32 / height as f32) * 2.0 - 1.0;
                let ny = -ny;

                let a = nx * nx + ny * ny - 0.3;

                if (a * a * a - nx * nx * ny * ny * ny) > 0.0 {
                    continue;
                }

                let px = start_x + x;
                let py = start_y + y;

                let pixel_index = ((py * frame_width + px) * 4) as usize;
                frame[pixel_index..pixel_index + 4].copy_from_slice(&color);

            }
        }
    }


    fn draw_circle(frame: &mut [u8], frame_width: u32, frame_height: u32){
        let height = 200;
        let width = 200;

        let center_x = frame_width/2;
        let center_y = frame_height/2;

        let start_x = center_x - width/2 - 100;
        let start_y = center_y - height/2;

        let color: [u8;4] =  [255,0,0,255];


        for y in 0..height{
            for x in 0..width{
                let nx = (x as f32 / width as f32) * 2.0 - 1.0;
                let ny = (y as f32 / height as f32) * 2.0 - 1.0;
                let ny = -ny;

                if (nx * nx+ ny * ny) > 0.5 {
                    continue;
                }

                let px = start_x + x;
                let py = start_y + y;

                let pixel_index = ((py * frame_width + px) * 4) as usize;
                frame[pixel_index..pixel_index + 4].copy_from_slice(&color);

            }
        }
    }

    //AI written cube********* I don't know how to do this.
    fn draw_cube(frame: &mut [u8], frame_width: u32, frame_height: u32) {
        let color = [255, 255, 255, 255];

        // Cube vertices
        let cube = [
            [-1.0, -1.0, -1.0],
            [ 1.0, -1.0, -1.0],
            [ 1.0,  1.0, -1.0],
            [-1.0,  1.0, -1.0],
            [-1.0, -1.0,  1.0],
            [ 1.0, -1.0,  1.0],
            [ 1.0,  1.0,  1.0],
            [-1.0,  1.0,  1.0],
        ];

        // Edges (pairs of vertex indices)
        let edges = [
            (0,1),(1,2),(2,3),(3,0),
            (4,5),(5,6),(6,7),(7,4),
            (0,4),(1,5),(2,6),(3,7),
        ];

        // Project 3D → 2D
        let mut projected = [[0i32; 2]; 8];
        let distance = 4.0;
        let scale = 180.0;

        for i in 0..8 {
            let x = cube[i][0];
            let y = cube[i][1];
            let z = cube[i][2] + distance;

            projected[i][0] = (frame_width as f32 / 2.0 + (x / z) * scale) as i32;
            projected[i][1] = (frame_height as f32 / 2.0 - (y / z) * scale) as i32;
        }

        // Draw lines (Bresenham)
        for &(a, b) in &edges {
            let mut x0 = projected[a][0];
            let mut y0 = projected[a][1];
            let x1 = projected[b][0];
            let y1 = projected[b][1];

            let dx = (x1 - x0).abs();
            let dy = -(y1 - y0).abs();
            let sx = if x0 < x1 { 1 } else { -1 };
            let sy = if y0 < y1 { 1 } else { -1 };
            let mut err = dx + dy;

            loop {
                if x0 >= 0 && y0 >= 0 && x0 < frame_width as i32 && y0 < frame_height as i32 {
                    let idx = ((y0 as u32 * frame_width + x0 as u32) * 4) as usize;
                    frame[idx..idx + 4].copy_from_slice(&color);
                }

                if x0 == x1 && y0 == y1 { break; }

                let e2 = 2 * err;
                if e2 >= dy { err += dy; x0 += sx; }
                if e2 <= dx { err += dx; y0 += sy; }
            }
        }
    }
}