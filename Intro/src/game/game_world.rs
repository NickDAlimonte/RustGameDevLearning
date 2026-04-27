use crate::game::player::Player;
use crate::game::tile::Tile;
pub struct GameWorld{
    player: Player,
    tiles: Vec<Tile>,
    width: u32,
    height: u32,



}
impl GameWorld{

    pub fn new() -> Self{
        let width = 10;
        let height = 10;
        let tilecount = width * height;

        let new_tile_vector: Vec<Tile> =
            (0..tilecount)
                .map(|_| Tile::default())
                .collect();

        let new_player = Player::new(true);


        GameWorld{player: new_player, tiles: new_tile_vector, width, height}
    }
}