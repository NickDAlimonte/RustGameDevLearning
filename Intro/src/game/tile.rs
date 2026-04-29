pub struct Tile{
    pub size: u32,
    pub color: [u8; 4],
}

impl Tile{

    pub fn new(size: u32, color: [u8; 4]) -> Self{
        Self{size, color}
    }
}

impl Default for Tile{
    fn default() -> Self {
        Self{size:0, color:[0; 4]}
    }
}