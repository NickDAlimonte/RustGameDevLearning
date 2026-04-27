pub struct Player{
    active: bool,

}

impl Player{
    pub fn new(active: bool)->Self{
        Self{active,}
    }
}