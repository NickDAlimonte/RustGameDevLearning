use crate::game::player::Player;
use crate::game::tile::Tile;
pub struct GameWorld{
    pub player: Player,
    pub tiles: Vec<Tile>,
    pub width: u32,
    pub height: u32,



}
impl GameWorld{

    pub fn new() -> Self{
        let width = 30;
        let height = 30;
        let tile_count = width * height;

        let new_tile_vector: Vec<Tile> =
            (0..tile_count)
                .map(|_| Tile::default())
                .collect();

        let new_player = Player::new(true);


        Self{player: new_player, tiles: new_tile_vector, width, height}
    }
}