pub struct Game {
    pub player_name: String,
}

impl Game {

    pub fn new(player: String) -> Self {
        Game {
            player_name: player,
        }
    }
}