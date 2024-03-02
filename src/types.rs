
pub enum Difficulty{
    Easy,
    Medium,
    Hard,
}

pub struct Tile {
    is_mine: bool,
    bombs_near: usize,
}

pub struct Board {

}

impl Board {
    pub fn new(difficulty: Difficulty) -> Self {
        todo!()
    }
}
